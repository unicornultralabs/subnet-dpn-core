use anyhow::{anyhow, Error, Result};
use log::{error, info};
use redis::{Commands as _, Connection, RedisResult};
use redis_async::client::{ConnectionBuilder, PubsubConnection};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug, sync::Arc};
use url::Url;

use crate::types::{bandwidth::UserBandwidthPrice, connection::ProxyAccData, task::UserTask};

use super::types::{PeerChanged, PeerChangedInfo, ProxyAccChanged};

struct RedisUri {
    is_tls: bool,
    password: Option<String>,
    host: String,
    port: u16,
}

#[derive(Debug)]
pub struct RedisService {
    client: redis::Client,
    pubsub_con: PubsubConnection,
}

impl RedisService {
    pub async fn new(redis_uri: String) -> Result<Self> {
        let client = redis::Client::open(redis_uri.clone())
            .map_err(|e| anyhow!("redis: cannot open client err={}", e))?;
        _ = client
            .get_connection()
            .map_err(|e| anyhow!("redis: cannot get connection err={}", e))?;

        let conn_builder = Self::get_redis_conn_builder_from_uri(&redis_uri)?;
        let pubsub_con = conn_builder
            .pubsub_connect()
            .await
            .map_err(|e| anyhow!("create pub sub connection failed err={}", e))?;

        Ok(Self { client, pubsub_con })
    }

    fn parse_redis_uri(redis_uri: &str) -> Result<RedisUri> {
        let parsed_url = Url::parse(redis_uri)?;

        let is_tls = match parsed_url.scheme() {
            "redis" => false,
            "rediss" => true,
            unknown => {
                return Err(anyhow::anyhow!(
                    "invalid scheme, must be 'redis' or 'rediss' unknown {}",
                    unknown
                ))
            }
        };
        let password = parsed_url.password().map(|p| p.to_string());

        let host = match parsed_url.host_str() {
            Some(host) => host.to_string(),
            None => return Err(anyhow::anyhow!("parse host failed")),
        };

        let port = match parsed_url.port() {
            Some(port) => port,
            None => return Err(anyhow::anyhow!("parse port failed")),
        };

        Ok(RedisUri {
            is_tls,
            password,
            host,
            port,
        })
    }

    pub fn get_redis_conn_builder_from_uri(redis_uri: &str) -> Result<ConnectionBuilder> {
        let redis_info =
            Self::parse_redis_uri(redis_uri).map_err(|e| anyhow!("parse failed err={}", e))?;

        let mut connection_builder: ConnectionBuilder =
            ConnectionBuilder::new(redis_info.host, redis_info.port)
                .map_err(|e| anyhow!("connection build create failed err={}", e))?;

        if redis_info.is_tls {
            connection_builder.tls();
        }

        if let Some(redis_password) = redis_info.password {
            connection_builder.password(redis_password);
        }

        Ok(connection_builder)
    }

    pub fn get_pubsub_conn(self: Arc<Self>) -> PubsubConnection {
        self.pubsub_con.clone()
    }

    pub fn hset<T>(self: Arc<Self>, key: String, field: String, obj: T) -> Result<(), Error>
    where
        T: Serialize,
    {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;
        match conn.hset::<String, String, String, usize>(
            key,
            field,
            serde_json::to_string(&obj).unwrap(),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("redis failed to insert err={}", e)),
        }
    }

    pub fn hset_with_ttl<T>(self: Arc<Self>, key: String, field: String, obj: T, ttl_seconds: u64) -> Result<(), Error>
    where
        T: Serialize,
    {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;
        
        // First set the hash field
        conn.hset::<String, String, String, usize>(
            key.clone(),
            field,
            serde_json::to_string(&obj).unwrap(),
        )
        .map_err(|e| anyhow!("redis failed to insert err={}", e))?;
        
        // Then set the expiration on the entire hash key
        conn.expire::<String, bool>(key.clone(), ttl_seconds as i64)
            .map_err(|e| anyhow!("redis failed to set expiration on key={} err={}", key, e))?;
        
        Ok(())
    }
    
    pub fn hget<T>(self: Arc<Self>, key: String, field: String) -> Result<T, Error>
    where
        T: Clone + DeserializeOwned,
    {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;
        let obj_str: String = conn
            .hget(key.clone(), field.clone())
            .map_err(|e| anyhow!("redis cannot get key={}:{} err={}", key, field, e))?;
        let t = serde_json::from_str::<T>(&obj_str)
            .map_err(|e| anyhow!("redis failed to decode err={}", e))?;
        Ok(t)
    }

    pub fn hgetall<T>(self: Arc<Self>, key: String) -> Result<Vec<(String, T)>, Error>
    where
        T: Clone + DeserializeOwned,
    {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;
        let result: HashMap<String, String> = conn
            .hgetall(key.clone())
            .map_err(|e| anyhow!("redis cannot get key={} err={}", key, e))?;
        let mut rs: Vec<(String, T)> = vec![];
        for (key, obj_str) in result.iter() {
            let proxy_acc = serde_json::from_str::<T>(&obj_str)
                .map_err(|e| anyhow!("redis failed to decode err={}", e))?;
            rs.push((key.clone(), proxy_acc.clone()));
        }
        Ok(rs)
    }

    pub fn hdel(self: Arc<Self>, key: String, field: String) -> Result<(), Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;
        conn.hdel(key.clone(), field.clone())
            .map_err(|e| anyhow!("redis cannot hdel key={} field={} err={}", key, field, e))?;
        Ok(())
    }

    pub fn zadd(self: Arc<Self>, key: String, score: u32, value: u32) -> Result<(), Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;
        match conn.zadd::<String, u32, u32, ()>(key, value, score) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(
                "redis failed to insert peer into peer queue err={}",
                e
            )),
        }
    }

    pub fn zrem(self: Arc<Self>, key: String, value: u32) -> Result<(), anyhow::Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;

        match conn.zrem::<String, u32, usize>(key, value) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(
                "redis failed to remove peer in peer queue err={}",
                e
            )),
        }
    }

    pub fn zsetall(self: Arc<Self>, key: String, score: u32) -> Result<(), anyhow::Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;

        let elements: Vec<(u32, u32)> = conn
            .zrange_withscores(key.clone(), 0, -1)
            .map_err(|e| anyhow!("redis failed to get sorted set err={}", e))?;

        for (value, _) in elements {
            conn.zadd::<String, u32, u32, ()>(key.clone(), value, score)
                .map_err(|e| anyhow!("redis failed to set scores err={}", e))?;
        }

        Ok(())
    }

    pub fn zgetall(self: Arc<Self>, key: String) -> Result<Vec<(u32, u32)>, Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;

        let elements: Vec<(u32, u32)> = conn
            .zrange_withscores(key.clone(), 0, -1)
            .map_err(|e| anyhow!("redis failed to get peer queue err={}", e))?;

        let mut result: Vec<(u32, u32)> = elements
            .into_iter()
            .map(|(value, score)| (value, score))
            .collect();

        result.sort_by_key(|(_value, score)| *score);

        Ok(result)
    }

    /// this function is used to delete data of given key
    pub fn del(self: Arc<Self>, key: String) -> Result<(), Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;

        conn.del(key.clone())
            .map_err(|e| anyhow!("redis failed to delete key={} err={}", key, e))
    }

    pub async fn publish(self: Arc<Self>, chan_name: String, obj_str: String) -> Result<(), Error> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| anyhow!("cannot get connection err={}", e))?;
        conn.publish(&chan_name, &obj_str)?;
        Ok(())
    }

    pub async fn get_conn(self: Arc<Self>) -> RedisResult<Connection> {
        self.client.get_connection()
    }

    /// remove all peers in redis cache
    /// it must be called when shutting down masternode
    pub async fn remove_all_peers(self: Arc<Self>, masternode_id: String) -> anyhow::Result<()> {
        let (k, _) = DPNRedisKey::get_peers_kf(masternode_id.clone(), 0);
        let peers = self
            .clone()
            .hgetall::<PeerChangedInfo>(k.clone())
            .map_err(|e| anyhow!("redis get peers failed err={}", e))?;

        for (_, change) in peers {
            // publish peer to redis
            let change = PeerChanged::Disconnected(PeerChangedInfo {
                uuid: change.uuid.clone(),
                login_session_id: change.login_session_id.clone(),
                ip_u32: change.ip_u32,
            });

            if let Err(e) = self
                .clone()
                .publish(
                    DPNRedisKey::get_peers_chan(masternode_id.clone()),
                    serde_json::to_string(&change).unwrap(),
                )
                .await
            {
                return Err(anyhow!(
                    "redis peer status publish failed status={:?} err={}",
                    change,
                    e
                ));
            }
        }

        self.clone()
            .del(k)
            .map_err(|e| anyhow!("failed to remove peers from redis err={}", e))
    }

    pub async fn publish_peer(
        self: Arc<Self>,
        masternode_id: String,
        status: PeerChanged,
    ) -> anyhow::Result<()> {
        match status.clone() {
            PeerChanged::Connected(info) => {
                // add peer to redis hash
                let (k, f) = DPNRedisKey::get_peers_kf(masternode_id.clone(), info.ip_u32);
                if let Err(e) = self.clone().hset(k, f, info.clone()) {
                    return Err(anyhow!("redis peer add failed err={}", e));
                }
            }
            PeerChanged::Disconnected(info) => {
                // remove peer from redis hash
                let (k, f) = DPNRedisKey::get_peers_kf(masternode_id.clone(), info.ip_u32);
                if let Err(e) = self.clone().hdel(k, f) {
                    return Err(anyhow!("redis peer removal failed err={}", e));
                }
            }
        };

        if let Err(e) = self
            .clone()
            .publish(
                DPNRedisKey::get_peers_chan(masternode_id.clone()),
                serde_json::to_string(&status).unwrap(),
            )
            .await
        {
            return Err(anyhow!(
                "redis peer status publish failed status={:?} err={}",
                status,
                e
            ));
        }

        Ok(())
    }

    pub async fn get_peers(self: Arc<Self>, masternode_id: String) -> Result<Vec<PeerChangedInfo>> {
        let (k, _) = DPNRedisKey::get_peers_kf(masternode_id, 0);
        let peers = self
            .clone()
            .hgetall::<PeerChangedInfo>(k)
            .map_err(|e| anyhow!("redis get peers failed err={}", e))?;
        Ok(peers
            .iter()
            .map(|(_, peer_info)| peer_info.clone())
            .collect())
    }

    pub async fn publish_peer_price(
        self: Arc<Self>,
        price: UserBandwidthPrice,
    ) -> anyhow::Result<()> {
        let (k, f) = DPNRedisKey::get_price_kf(price.user_addr.clone());
        self.clone()
            .hset(k, f, price.clone())
            .map_err(|e| anyhow!("redis set peer price failed err={}", e))?;

        self.clone()
            .publish(
                DPNRedisKey::get_price_chan(),
                serde_json::to_string(&price).unwrap(),
            )
            .await
            .map_err(|e| {
                anyhow!(
                    "redis peer status publish failed price={:?} err={}",
                    price,
                    e
                )
            })?;
        Ok(())
    }

    pub async fn publish_first_time_provider(
        self: Arc<Self>,
        provider: UserTask,
    ) -> anyhow::Result<()> {
        let (k, f) = DPNRedisKey::get_first_time_provider_kf(provider.user_addr.clone());
        self.clone()
            .hset(k, f, provider.clone())
            .map_err(|e| anyhow!("redis set first time provider failed err={}", e))?;

        self.clone()
            .publish(
                DPNRedisKey::get_first_time_provider_chan(),
                serde_json::to_string(&provider).unwrap(),
            )
            .await
            .map_err(|e| {
                anyhow!(
                    "redis first time provider publish failed err={}",
                    e
                )
            })?;
        Ok(())
    }

    pub async fn publish_withdrawal_reward(
        self: Arc<Self>,
        provider: UserTask,
    ) -> anyhow::Result<()> {
        let (k, f) = DPNRedisKey::get_withdrawal_reward_kf(provider.user_addr.clone());
        self.clone()
            .hset(k, f, provider.clone())
            .map_err(|e| anyhow!("redis set withdrawal reward failed err={}", e))?;

        self.clone()
            .publish(
                DPNRedisKey::get_withdrawal_reward_chan(),
                serde_json::to_string(&provider).unwrap(),
            )
            .await
            .map_err(|e| {
                anyhow!(
                    "redis first time provider publish failed err={}",
                    e
                )
            })?;
        Ok(())
    }

    pub async fn publish_completed_8_hours_ot(
        self: Arc<Self>,
        provider: UserTask,
    ) -> anyhow::Result<()> {
        let (k, f) = DPNRedisKey::get_completed_8_hours_ot_kf(provider.user_addr.clone());
        self.clone()
            .hset(k, f, provider.clone())
            .map_err(|e| anyhow!("redis set completed 8 hours ot failed err={}", e))?;

        self.clone()
            .publish(
                DPNRedisKey::get_completed_8_hours_ot_chan(),
                serde_json::to_string(&provider).unwrap(),
            )
            .await
            .map_err(|e| {
                anyhow!(
                    "redis completed 8 hours ot publish failed err={}",
                    e
                )
            })?;
        Ok(())
    }
    


    pub async fn publish_invite_friend_one_time(
        self: Arc<Self>,
        provider: UserTask,
    ) -> anyhow::Result<()> {
        self.clone()
            .publish(
                DPNRedisKey::get_invite_friend_one_time_chan(),
                serde_json::to_string(&provider).unwrap(),
            )
            .await
            .map_err(|e| {
                anyhow!(
                    "redis invite friend one time publish failed err={}",
                    e
                )
            })?;
        Ok(())
    }

    pub async fn publish_completed_time_per_day(
        self: Arc<Self>,
        provider: UserTask,
    ) -> anyhow::Result<()> {
        let (k, f) = DPNRedisKey::get_completed_time_per_day_kf(provider.user_addr.clone());
        self.clone()
        .hset(k, f, provider.clone())
        .map_err(|e| anyhow!("redis set completed time per day failed err={}", e))?;
        self.clone()
            .publish(
                DPNRedisKey::get_completed_time_per_day_chan(),
                serde_json::to_string(&provider).unwrap(),
            )
            .await
    }

    pub async fn get_peers_price(self: Arc<Self>) -> Result<Vec<UserBandwidthPrice>> {
        let (k, _) = DPNRedisKey::get_price_kf("".to_string());
        let peers = self
            .clone()
            .hgetall::<UserBandwidthPrice>(k)
            .map_err(|e| anyhow!("redis get peers price failed err={}", e))?;
        Ok(peers
            .iter()
            .map(|(_, peer_info)| peer_info.clone())
            .collect())
    }

    pub async fn get_proxy_accs(self: Arc<Self>) -> Result<Vec<ProxyAccData>> {
        let (k, _) = DPNRedisKey::get_proxy_acc_kf("".to_string());
        let proxy_accs = self
            .clone()
            .hgetall::<ProxyAccData>(k)
            .map_err(|e| anyhow!("redis get proxy accs failed err={}", e))?;
        Ok(proxy_accs.iter().map(|(_, pad)| pad.clone()).collect())
    }

    /// remove all proxy accs in redis cache
    /// it must be called when admin started
    /// after removal, proxy accs are loaded from db and added to redis
    pub async fn remove_all_proxy_accs(self: Arc<Self>) -> anyhow::Result<()> {
        let (k, _) = DPNRedisKey::get_proxy_acc_kf("".to_owned());
        self.clone()
            .del(k)
            .map_err(|e| anyhow!("failed to remove peers from redis err={}", e))
    }

    /// remove all withdrawal reward in redis cache
    /// it must be called when admin started
    /// after removal, withdrawal reward are loaded from db and added to redis
    pub async fn remove_all_withdrawal_reward(self: Arc<Self>) -> anyhow::Result<()> {
        let (k, _) = DPNRedisKey::get_withdrawal_reward_kf("".to_owned());
        self.clone()
            .del(k)
            .map_err(|e| anyhow!("failed to remove withdrawal reward from redis err={}", e))
    }
    
    pub async fn publish_proxy_acc(
        self: Arc<Self>,
        proxy_acc_changed: ProxyAccChanged,
    ) -> anyhow::Result<()> {
        match proxy_acc_changed.clone() {
            ProxyAccChanged::Created(pad) => {
                let (k, f) = DPNRedisKey::get_proxy_acc_kf(pad.id.clone());
                self.clone()
                    .hset(k, f, pad.clone())
                    .map_err(|e| anyhow!("{}", e))?;
            }
            ProxyAccChanged::Updated(pad) => {
                let (k, f) = DPNRedisKey::get_proxy_acc_kf(pad.id.clone());
                self.clone()
                    .hset(k, f, pad.clone())
                    .map_err(|e| anyhow!("{}", e))?;
            }
            ProxyAccChanged::Deleted(id) => {
                let (k, f) = DPNRedisKey::get_proxy_acc_kf(id.clone());
                self.clone().hdel(k, f).map_err(|e| anyhow!("{}", e))?;
            }
            ProxyAccChanged::RefreshAll() => { /**/ }
        }

        if let Err(e) = self
            .clone()
            .publish(
                DPNRedisKey::get_proxy_acc_chan(),
                serde_json::to_string(&proxy_acc_changed).unwrap(),
            )
            .await
        {
            return Err(anyhow!(
                "redis proxy acc publish failed change={:?} err={}",
                proxy_acc_changed,
                e
            ));
        }

        Ok(())
    }
}

pub struct DPNRedisKey {}
impl DPNRedisKey {
    pub fn get_geo_kf(masternode_id: String, login_session_id: String) -> (String, String) {
        (
            "peer_geo".to_owned(),
            format!("{}_{}", masternode_id.clone(), login_session_id.clone()),
        )
    }

    pub fn get_balance_kf(user_addr: String) -> (String, String) {
        (
            "client_user_balance".to_owned(),
            format!("{}", user_addr),
        )
    }

    pub fn get_peer_queue_k(masternode_id: String) -> String {
        format!("peer_queue_ms#{}_", masternode_id)
    }

    pub fn get_peers_kf(masternode_id: String, ip_u32: u32) -> (String, String) {
        (format!("peers_ms#{}", masternode_id), format!("{}", ip_u32))
    }

    pub fn get_peers_chan(masternode_id: String) -> String {
        format!("peers_updated_ms#{}", masternode_id)
    }

    pub fn get_price_kf(peer_addr: String) -> (String, String) {
        ("peer_price".to_owned(), peer_addr)
    }

    pub fn get_proxy_acc_kf(id: String) -> (String, String) {
        ("proxy_acc".to_owned(), id)
    }

    pub fn get_uptime_xp_kf(id: String) -> (String, String) {
        ("uptime_xp".to_owned(), id)
    }

    pub fn get_proxy_acc_chan() -> String {
        "proxy_acc_updated".to_string()
    }

    pub fn get_price_chan() -> String {
        "price_updated".to_string()
    }

    pub fn get_bonus_config_hash_key() -> String {
        "bonus_config".to_string()
    }

    pub fn get_first_time_provider_kf(id: String) -> (String, String) {
        ("first_time_provider".to_owned(), id)
    }

    pub fn get_first_time_provider_chan() -> String {
        "first_time_provider_updated".to_string()
    }

    pub fn get_withdrawal_reward_kf(id: String) -> (String, String) {
        ("withdrawal_reward".to_owned(), id)
    }

    pub fn get_withdrawal_reward_chan() -> String {
        "withdrawal_reward_updated".to_string()
    }

    pub fn get_completed_8_hours_ot_kf(id: String) -> (String, String) {
        ("completed_8_hours_ot".to_owned(), id)
    }

    pub fn get_completed_8_hours_ot_chan() -> String {
        "completed_8_hours_dl_updated".to_string()
    }

    pub fn get_invite_friend_one_time_kf(id: String) -> (String, String) {
        ("invite_friend_one_time".to_owned(), id)
    }

    pub fn get_invite_friend_one_time_chan() -> String {
        "invite_friend_one_time_updated".to_string()
    }

    pub fn get_completed_time_per_day_kf(id: String) -> (String, String) {
        ("completed_time_per_day".to_owned(), id)
    }
    
    pub fn get_completed_time_per_day_chan() -> String {
        "completed_time_per_day_updated".to_string()
    }

    pub fn get_user_addr_geo_kf(user_addr: String) -> (String, String) {
        ("user_addr_geo".to_owned(), user_addr)
    }

    // pub fn get_total_refers_one_time_kf(id: String) -> (String, String) {
    //     ("total_refers_one_time".to_owned(), id)
    // }

    // pub fn get_total_refers_one_time_chan() -> String {
    //     "total_refers_one_time_updated".to_string()
    // }


}
