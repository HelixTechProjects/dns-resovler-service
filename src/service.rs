use crate::contracts::dns_contract::DnsContract;
use crate::resolver::{resolve_all, resolve_only_http};
use crate::x_com::source_api::{ResolveInfo, ResolveRequest};
use std::collections::HashMap;
use std::sync::atomic::AtomicI64;
use std::sync::Mutex;
use x_com_lib::status_err;
use x_com_lib::x_api::xport::{build_message_channel, get_channel_id_by_conn_id};
use x_com_lib::x_core::xrpc::Context;
use x_com_lib::x_core::{self, config};

pub struct DnsResolverService {
    pub(crate) dns_url: String,
    pub(crate) dns_node_id: AtomicI64,

    pub(crate) dns_contract: Box<DnsContract>,
    // 域名解析缓存
    pub(crate) domain_parse_cache_map: Mutex<HashMap<String, i64>>,
    //
}

impl DnsResolverService {
    pub fn new() -> DnsResolverService {
        DnsResolverService {
            dns_url: String::default(),
            dns_contract: Box::new(DnsContract::new()),
            dns_node_id: AtomicI64::new(0),
            domain_parse_cache_map: Mutex::new(HashMap::new()),
        }
    }

    //
    pub async fn on_init(&mut self) -> x_core::Result<()> {
        if let Some(dns_url_str) = config::get_str("dns-url") {
            let Some(dns_addr) = config::get_str("dns-addr") else {
                return status_err!("缺少配置: dns-addr");
            };
            let conn_id = resolve_only_http(dns_url_str).await?;
            self.dns_contract.set_address(dns_addr.to_string());
            //
            self.dns_node_id
                .store(conn_id, std::sync::atomic::Ordering::SeqCst);
            //
            self.dns_url = dns_url_str.to_string();
            //
            build_message_channel(conn_id).await?;
        }

        Ok(())
    }

    pub async fn on_finalize(&self) {
        //
    }

    pub(crate) async fn refresh_dns_node_id(&self) -> x_core::Result<i64> {
        let conn_id = resolve_only_http(&self.dns_url).await?;
        //  进阿里
        self.dns_node_id
            .store(conn_id, std::sync::atomic::Ordering::SeqCst);
        Ok(conn_id)
    }

    pub(crate) fn get_dns_node_id(&self) -> i64 {
        self.dns_node_id.load(std::sync::atomic::Ordering::Relaxed)
    }

    // Resolve
    pub async fn resolve(
        &self,
        _ctx: Context,
        param: Box<ResolveRequest>,
    ) -> x_core::Result<Box<ResolveInfo>> {
        // 会有并发，不过不重要
        let mut conn_id = {
            let dns_url_conn_id_map = self.domain_parse_cache_map.lock().unwrap();
            if let Some(&conn_id) = dns_url_conn_id_map.get(&param.url) {
                conn_id
            } else {
                0
            }
        };

        //
        let mut resp = Box::new(ResolveInfo::default());
        if conn_id == 0 {
            conn_id = if self.dns_node_id.load(std::sync::atomic::Ordering::Relaxed) == 0 {
                resolve_only_http(&param.url).await?
            } else {
                resolve_all(&param.url).await?
            };
        }
        //

        if conn_id == 0 {
            return Ok(resp);
        }
        //
        {
            let mut dns_url_conn_id_map = self.domain_parse_cache_map.lock().unwrap();
            dns_url_conn_id_map.insert(param.url, conn_id);
        }
        //

        resp.conn_id = conn_id;
        if !param.build_channel {
            return Ok(resp);
        }
        //

        let channel_id = get_channel_id_by_conn_id(conn_id).await?;
        if let Some(channel_id) = channel_id.channel_id {
            resp.channel_id = channel_id;
            return Ok(resp);
        }
        // 创建
  
        build_message_channel(conn_id).await?;
        let channel_id = get_channel_id_by_conn_id(conn_id).await?;
        if let Some(channel_id) = channel_id.channel_id {
            resp.channel_id = channel_id;
        }
        Ok(resp)
    }
}
