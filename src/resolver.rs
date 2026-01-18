use std::net::IpAddr;

use crate::x_com::{
    source_api::{CallData, CallResponse},
    xport_core::get_service,
};
use bcos_lib::namehash;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};
use url::Url;
use x_com_lib::{
    make_node_id, status_err,
    x_api::{
        self,
        xport::{build_message_channel, get_channel_id_by_conn_id},
    },
    x_core::{
        self, add_request_handler, build_request_future, parse_response_and_wake, serial_request,
    }, TargetKey,
};
async fn lookup_http_ip(domain: &str) -> x_core::Result<String> {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
    let ips = resolver.lookup_ip(domain).await;

    if let Err(err) = ips {
        return status_err!(err.to_string());
    }
    let ips = ips.unwrap();

    for ip in ips.iter() {
        return Ok(ip.to_string());
    }
    Ok(String::default())
}

async fn lookup_dxm_ip(domain: &str) -> x_core::Result<String> {
    //
    let service = get_service();
    let node_id = service.get_dns_node_id();
    let channel_id = get_channel_id_by_conn_id(node_id).await?;
    let channel_id = if let Some(channel_id) = channel_id.channel_id {
        channel_id
    } else {
        let node_id = service.refresh_dns_node_id().await?;
        build_message_channel(node_id).await?;
        let channel_id = get_channel_id_by_conn_id(node_id).await?;
        let Some(channel_id) = channel_id.channel_id else {
            return status_err!("解析域名失败！");
        };
        channel_id
    };

    let domain_bytes = namehash(domain);

    let call_data = service.dns_contract.get_ip_encode(domain_bytes.to_vec())?;

    let mut param = Box::new(CallData::default());
    param.contract_addr = call_data.0;
    param.data = call_data.1;

    let (future, request_id) = build_request_future();
    let buffer = serial_request(&param);
    let clone_shared_state = future.shared_state.clone();
    add_request_handler(
        request_id,
        Box::new(move |buffer| {
            parse_response_and_wake::<CallResponse>(&clone_shared_state, buffer);
        }),
    );
    //
    let mut receiver = TargetKey::default();
    receiver.dxc_name = "BcosService".into();
    receiver.dxc_version = "0.0.1".into();
    receiver.api = "SendCallData".to_owned();
    x_api::xport::send_message(request_id, receiver, channel_id, buffer);
    let ret = future.await?;
    let ip = service.dns_contract.get_ip_decode(ret.value)?;
    Ok(ip)
}

pub async fn resolve_only_http(dns_url_str: &str) -> x_core::Result<i64> {
    let dns_url = Url::parse(dns_url_str);
    if let Err(err) = dns_url {
        return status_err!("解析 dns-url 失败: {}", err.to_string());
    };
    //
    let dns_url = dns_url.unwrap();
    let domain = dns_url.host_str().unwrap();
    let ip_str = if domain.parse::<IpAddr>().is_ok() {
        domain.to_string()
    } else {
        if !matches!(dns_url.scheme(), "http" | "https") {
            return status_err!("dns-url 只能是 http 或 https");
        }
        lookup_http_ip(domain).await?
    };
    if ip_str.is_empty() {
        return status_err!("域名解析失败!");
    }
    //
    let port = dns_url.port().unwrap_or(8090);
    let node_id = make_node_id(&ip_str, port);

    Ok(node_id)
}

pub async fn resolve_all(dns_url_str: &str) -> x_core::Result<i64> {
    let dns_url = Url::parse(dns_url_str);
    if let Err(err) = dns_url {
        return status_err!("解析 dns-url 失败: {}", err.to_string());
    };
    //
    let dns_url = dns_url.unwrap();
    let domain = dns_url.host_str().unwrap();
    let ip_str = if domain.parse::<IpAddr>().is_ok() {
        domain.to_string()
    } else {
        if dns_url.scheme() == "dxm" {
            lookup_dxm_ip(domain).await?
        } else {
            lookup_http_ip(domain).await?
        }
    };
    if ip_str.is_empty() {
        return status_err!("域名解析失败!");
    }
    //
    let port = dns_url.port().unwrap_or(8090);
    let node_id = make_node_id(&ip_str, port);

    Ok(node_id)
}
