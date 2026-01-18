#![allow(unused_imports)]
#![allow(static_mut_refs)]

use crate::service::DnsResolverService;
use core::slice;
use x_com_lib::status_err;
use x_com_lib::x_core;
use x_com_lib::x_core::gen_id;
use x_com_lib::x_core::parse_request_param;
use x_com_lib::x_core::response_empty_msg;
use x_com_lib::x_core::response_msg;
use x_com_lib::x_core::set_request_id;
use x_com_lib::x_core::xrpc;
use x_com_lib::x_core::{get_runtime, init_runtime, take_runtime};
use x_com_lib::CodedInputStream;
use x_com_lib::ProtocolDXCReader;
use x_com_lib::Status;
pub static mut SERVICE: Option<Box<DnsResolverService>> = None;
pub fn get_service() -> &'static DnsResolverService {
    unsafe { SERVICE.as_ref().unwrap() }
}

#[no_mangle]
pub extern "C" fn init(service_id: i64, config: *const u8, config_len: u32, log_level: i32) {
    init_runtime();
    let runtime = get_runtime();
    // 初始化日志
    runtime.block_on(async {
        let request_id = gen_id();
        set_request_id(request_id);
        let config_str = unsafe {
            let buffer = slice::from_raw_parts(config as *mut u8, config_len as usize);
            std::str::from_utf8_unchecked(buffer)
        };
        x_core::init_app(service_id, "DnsResolverService", &config_str, log_level);
        let service_ins = Box::new(DnsResolverService::new());
        unsafe {
            SERVICE = Some(service_ins);
            let ret = SERVICE.as_mut().unwrap().on_init().await;
            if ret.is_err() {
                response_empty_msg(0, &ret);
                return;
            }
        }
        set_request_id(0);
        let ok = Ok(());
        response_empty_msg(0, &ok);
    });
}
#[no_mangle]
pub extern "C" fn finalize() {
    let runtime = take_runtime();
    runtime.block_on(async {
        let request_id = gen_id();
        set_request_id(request_id);
        unsafe {
            let service = SERVICE.take();
            service.unwrap().on_finalize().await;
        }
        set_request_id(0);
    });
}
#[no_mangle]
pub extern "C" fn dispatch_message(buffer: *const u8, buffer_len: u32) {
    let vec_buffer = unsafe { slice::from_raw_parts(buffer as *mut u8, buffer_len as usize) };

    let _dxc_msg_reader = ProtocolDXCReader::new(vec_buffer);
    let msg_header = _dxc_msg_reader.header();
    let msg_body = _dxc_msg_reader.msg_body();
    let _ctx = xrpc::Context {
        sender_service_key: msg_header.sender_key,
        channel_id: msg_header.channel_id,
        conn_id: msg_header.conn_id,
        request_id: msg_header.request_id,
        from_addr: msg_header.from_address,
    };
    let Some(receiver_key) = msg_header.receiver_key else {
        let err_status = status_err!("数据格式出错！");
        response_empty_msg(msg_header.request_id, &err_status);
        return;
    };
    let mut _input_stream = CodedInputStream::from_bytes(msg_body);
    match receiver_key.api.as_str() {
        "Resolve" => {
            let param = parse_request_param(&mut _input_stream);
            x_core::spawn(async move {
                let service = get_service();
                let result = service.resolve(_ctx, param).await;
                response_msg(msg_header.request_id, &result);
            });
        }
        _ => {
            let err_status = status_err!("{} 不存在！", receiver_key.api);
            response_empty_msg(msg_header.request_id, &err_status);
        }
    }
}
