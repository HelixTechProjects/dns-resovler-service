#![allow(dead_code)]
#![allow(unused)]

use bcos_lib::{
    build_raw_log,
    entity::{LogEntry, Transaction, TransactionReceipt},
    ethabi::{self, ethereum_types::*, *},
    hex::ToHex,
    hex_addr_to_h160, parse_event,
    utils::hex_encode_with_prefix,
};
use std::{io::Cursor, sync::Arc};
use x_com_lib::{status_err, x_core, Status};

pub struct DnsContract {
    pub contract_addr: String,
    name_func: Function,
    add_manager_func: Function,
    get_ip_func: Function,
    register_sub_domain_func: Function,
    register_domain_func: Function,
    remove_manager_func: Function,
    update_ip_func: Function,
    is_manager_func: Function,
}
impl DnsContract {
    pub fn new() -> Self {
        let abi_str: &str = r#"[{"constant":true,"inputs":[],"name":"name","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_managerAddr","type":"address"}],"name":"addManager","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"_domain","type":"bytes32"}],"name":"getIp","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_domain","type":"bytes32"},{"name":"_label","type":"bytes32"}],"name":"registerSubDomain","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"_domain","type":"bytes32"},{"name":"_ownerAddr","type":"address"}],"name":"registerDomain","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"_managerAddr","type":"address"}],"name":"removeManager","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"_domain","type":"bytes32"},{"name":"_ip","type":"string"}],"name":"updateIp","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"_managerAddr","type":"address"}],"name":"isManager","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"view","type":"function"},{"inputs":[],"payable":false,"stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":false,"name":"_domain","type":"bytes32"},{"indexed":false,"name":"ip","type":"string"}],"name":"UpdateIp","type":"event"}]"#;
        let abi_bytes = abi_str.as_bytes();
        let abi_cursor = Cursor::new(abi_bytes);
        let contract_abi = ethabi::Contract::load(abi_cursor).unwrap();

        let name_func = contract_abi.function("name").unwrap().clone();
        let add_manager_func = contract_abi.function("addManager").unwrap().clone();
        let get_ip_func = contract_abi.function("getIp").unwrap().clone();
        let register_sub_domain_func = contract_abi.function("registerSubDomain").unwrap().clone();
        let register_domain_func = contract_abi.function("registerDomain").unwrap().clone();
        let remove_manager_func = contract_abi.function("removeManager").unwrap().clone();
        let update_ip_func = contract_abi.function("updateIp").unwrap().clone();
        let is_manager_func = contract_abi.function("isManager").unwrap().clone();

        DnsContract {
            contract_addr: String::default(),
            name_func,
            add_manager_func,
            get_ip_func,
            register_sub_domain_func,
            register_domain_func,
            remove_manager_func,
            update_ip_func,
            is_manager_func,
        }
    }
    pub fn set_address(&mut self, contract_addr: String) {
        self.contract_addr = contract_addr;
    }
    pub fn name_encode(&self) -> x_core::Result<(String, String)> {
        let params: Vec<u8> = self.name_func.encode_input(&[]).unwrap();
        let call_data = bcos_lib::hex::encode(params);
        Ok((self.contract_addr.clone(), call_data))
    }
    pub fn name_decode(&self, result: String) -> x_core::Result<String> {
        let result = &result.as_bytes()[2..];
        let result = bcos_lib::hex::decode(result);
        let Ok(result) = result else {
            return status_err!("解析参数失败");
        };
        let Ok(mut output_vals) = self.name_func.decode_output(&result) else {
            return status_err!("解析参数失败");
        };
        let val_0 = output_vals.remove(0).into_string().unwrap();
        Ok(val_0)
    }
    pub fn add_manager(
        &self,
        block_number: i64,
        _manager_addr: String,
    ) -> x_core::Result<Transaction> {
        let _manager_addr = hex_addr_to_h160(&_manager_addr)?;
        let params: Vec<u8> = self
            .add_manager_func
            .encode_input(&[Token::Address(_manager_addr)])
            .unwrap();
        let mut tx: Transaction = Transaction::default();
        tx.data = Some(params);
        tx.to = self.contract_addr.clone();
        tx.block_limit = block_number + 500;
        Ok(tx)
    }
    pub fn get_ip_encode(&self, _domain: Vec<u8>) -> x_core::Result<(String, String)> {
        let params: Vec<u8> = self
            .get_ip_func
            .encode_input(&[Token::FixedBytes(_domain)])
            .unwrap();
        let call_data = bcos_lib::hex::encode(params);
        Ok((self.contract_addr.clone(), call_data))
    }
    pub fn get_ip_decode(&self, result: String) -> x_core::Result<String> {
        let result = &result.as_bytes()[2..];
        let result = bcos_lib::hex::decode(result);
        let Ok(result) = result else {
            return status_err!("解析参数失败");
        };
        let Ok(mut output_vals) = self.get_ip_func.decode_output(&result) else {
            return status_err!("解析参数失败");
        };
        let val_0 = output_vals.remove(0).into_string().unwrap();
        Ok(val_0)
    }
    pub fn register_sub_domain(
        &self,
        block_number: i64,
        _domain: Vec<u8>,
        _label: Vec<u8>,
    ) -> x_core::Result<Transaction> {
        let params: Vec<u8> = self
            .register_sub_domain_func
            .encode_input(&[Token::FixedBytes(_domain), Token::FixedBytes(_label)])
            .unwrap();
        let mut tx: Transaction = Transaction::default();
        tx.data = Some(params);
        tx.to = self.contract_addr.clone();
        tx.block_limit = block_number + 500;
        Ok(tx)
    }
    pub fn register_domain(
        &self,
        block_number: i64,
        _domain: Vec<u8>,
        _owner_addr: String,
    ) -> x_core::Result<Transaction> {
        let _owner_addr = hex_addr_to_h160(&_owner_addr)?;
        let params: Vec<u8> = self
            .register_domain_func
            .encode_input(&[Token::FixedBytes(_domain), Token::Address(_owner_addr)])
            .unwrap();
        let mut tx: Transaction = Transaction::default();
        tx.data = Some(params);
        tx.to = self.contract_addr.clone();
        tx.block_limit = block_number + 500;
        Ok(tx)
    }
    pub fn remove_manager(
        &self,
        block_number: i64,
        _manager_addr: String,
    ) -> x_core::Result<Transaction> {
        let _manager_addr = hex_addr_to_h160(&_manager_addr)?;
        let params: Vec<u8> = self
            .remove_manager_func
            .encode_input(&[Token::Address(_manager_addr)])
            .unwrap();
        let mut tx: Transaction = Transaction::default();
        tx.data = Some(params);
        tx.to = self.contract_addr.clone();
        tx.block_limit = block_number + 500;
        Ok(tx)
    }
    pub fn update_ip(
        &self,
        block_number: i64,
        _domain: Vec<u8>,
        _ip: String,
    ) -> x_core::Result<Transaction> {
        let params: Vec<u8> = self
            .update_ip_func
            .encode_input(&[Token::FixedBytes(_domain), Token::String(_ip)])
            .unwrap();
        let mut tx: Transaction = Transaction::default();
        tx.data = Some(params);
        tx.to = self.contract_addr.clone();
        tx.block_limit = block_number + 500;
        Ok(tx)
    }
    pub fn is_manager_encode(&self, _manager_addr: String) -> x_core::Result<(String, String)> {
        let _manager_addr = hex_addr_to_h160(&_manager_addr)?;
        let params: Vec<u8> = self
            .is_manager_func
            .encode_input(&[Token::Address(_manager_addr)])
            .unwrap();
        let call_data = bcos_lib::hex::encode(params);
        Ok((self.contract_addr.clone(), call_data))
    }
    pub fn is_manager_decode(&self, result: String) -> x_core::Result<bool> {
        let result = &result.as_bytes()[2..];
        let result = bcos_lib::hex::decode(result);
        let Ok(result) = result else {
            return status_err!("解析参数失败");
        };
        let Ok(mut output_vals) = self.is_manager_func.decode_output(&result) else {
            return status_err!("解析参数失败");
        };
        let val_0 = output_vals.remove(0).into_bool().unwrap();
        Ok(val_0)
    }
}
