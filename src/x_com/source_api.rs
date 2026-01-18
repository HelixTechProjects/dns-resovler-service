#[allow(unused_imports)]
use super::import_api::*;
#[allow(unused_imports)]
use x_com_lib::{
    self, extract_wire_type_from_tag, pb_error_to_status,
    x_core::{self, serial_request},
    CodedInputStream, CodedOutputStream, RequestMessage, Status,
};

#[derive(Clone)]
pub struct ResolveInfo {
    pub conn_id: i64,
    pub channel_id: i64,
    pub(crate) cached_size: x_com_lib::rt::CachedSize,
}

#[derive(Clone)]
pub struct CallResponse {
    pub value: String,
    pub(crate) cached_size: x_com_lib::rt::CachedSize,
}

#[derive(Clone)]
pub struct CallData {
    pub contract_addr: String,
    pub data: String,
    pub(crate) cached_size: x_com_lib::rt::CachedSize,
}

#[derive(Clone)]
pub struct ResolveRequest {
    pub url: String,
    pub build_channel: bool,
    pub(crate) cached_size: x_com_lib::rt::CachedSize,
}

impl std::fmt::Debug for ResolveRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResolveRequest")
            .field("url", &self.url)
            .field("build_channel", &self.build_channel)
            .finish()
    }
}

impl Default for ResolveRequest {
    fn default() -> Self {
        ResolveRequest {
            url: String::default(),
            build_channel: false,
            cached_size: x_com_lib::rt::CachedSize::new(),
        }
    }
}

impl RequestMessage for ResolveRequest {
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.url.is_empty() {
            my_size += x_com_lib::rt::string_size(1, &self.url);
        }
        if self.build_channel != false {
            my_size += 2;
        }
        self.cached_size.set(my_size as u32);
        my_size
    }

    fn serial_with_output_stream(
        &self,
        os: &mut x_com_lib::CodedOutputStream<'_>,
    ) -> Result<(), Status> {
        if !self.url.is_empty() {
            os.write_string(1, &self.url).unwrap();
        }
        if self.build_channel != false {
            os.write_bool(2, self.build_channel).unwrap();
        }
        Ok(())
    }

    fn parse_from_input_stream(
        &mut self,
        is: &mut x_com_lib::CodedInputStream<'_>,
    ) -> Result<(), Status> {
        while let Some(tag) = is.read_raw_tag_or_eof().unwrap() {
            match tag {
                10 => {
                    self.url = is.read_string().map_err(pb_error_to_status)?;
                }
                16 => {
                    self.build_channel = is.read_bool().map_err(pb_error_to_status)?;
                }
                _ => {
                    let wire_type = extract_wire_type_from_tag(tag);
                    if wire_type.is_none() {
                        return Err(Status::error("消息格式出错".into()));
                    }
                    let result = is.skip_field(wire_type.unwrap());
                    if let Err(err) = result {
                        return Err(Status::error(err.to_string()));
                    }
                }
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for ResolveInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResolveInfo")
            .field("conn_id", &self.conn_id)
            .field("channel_id", &self.channel_id)
            .finish()
    }
}

impl Default for ResolveInfo {
    fn default() -> Self {
        ResolveInfo {
            conn_id: 0,
            channel_id: 0,
            cached_size: x_com_lib::rt::CachedSize::new(),
        }
    }
}

impl RequestMessage for ResolveInfo {
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.conn_id != 0 {
            my_size += x_com_lib::rt::int64_size(1, self.conn_id);
        }
        if self.channel_id != 0 {
            my_size += x_com_lib::rt::int64_size(2, self.channel_id);
        }
        self.cached_size.set(my_size as u32);
        my_size
    }

    fn serial_with_output_stream(
        &self,
        os: &mut x_com_lib::CodedOutputStream<'_>,
    ) -> Result<(), Status> {
        if self.conn_id != 0 {
            os.write_int64(1, self.conn_id).unwrap();
        }
        if self.channel_id != 0 {
            os.write_int64(2, self.channel_id).unwrap();
        }
        Ok(())
    }

    fn parse_from_input_stream(
        &mut self,
        is: &mut x_com_lib::CodedInputStream<'_>,
    ) -> Result<(), Status> {
        while let Some(tag) = is.read_raw_tag_or_eof().unwrap() {
            match tag {
                8 => {
                    self.conn_id = is.read_int64().map_err(pb_error_to_status)?;
                }
                16 => {
                    self.channel_id = is.read_int64().map_err(pb_error_to_status)?;
                }
                _ => {
                    let wire_type = extract_wire_type_from_tag(tag);
                    if wire_type.is_none() {
                        return Err(Status::error("消息格式出错".into()));
                    }
                    let result = is.skip_field(wire_type.unwrap());
                    if let Err(err) = result {
                        return Err(Status::error(err.to_string()));
                    }
                }
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for CallResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallResponse")
            .field("value", &self.value)
            .finish()
    }
}

impl Default for CallResponse {
    fn default() -> Self {
        CallResponse {
            value: String::default(),
            cached_size: x_com_lib::rt::CachedSize::new(),
        }
    }
}

impl RequestMessage for CallResponse {
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.value.is_empty() {
            my_size += x_com_lib::rt::string_size(1, &self.value);
        }
        self.cached_size.set(my_size as u32);
        my_size
    }

    fn serial_with_output_stream(
        &self,
        os: &mut x_com_lib::CodedOutputStream<'_>,
    ) -> Result<(), Status> {
        if !self.value.is_empty() {
            os.write_string(1, &self.value).unwrap();
        }
        Ok(())
    }

    fn parse_from_input_stream(
        &mut self,
        is: &mut x_com_lib::CodedInputStream<'_>,
    ) -> Result<(), Status> {
        while let Some(tag) = is.read_raw_tag_or_eof().unwrap() {
            match tag {
                10 => {
                    self.value = is.read_string().map_err(pb_error_to_status)?;
                }
                _ => {
                    let wire_type = extract_wire_type_from_tag(tag);
                    if wire_type.is_none() {
                        return Err(Status::error("消息格式出错".into()));
                    }
                    let result = is.skip_field(wire_type.unwrap());
                    if let Err(err) = result {
                        return Err(Status::error(err.to_string()));
                    }
                }
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for CallData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallData")
            .field("contract_addr", &self.contract_addr)
            .field("data", &self.data)
            .finish()
    }
}

impl Default for CallData {
    fn default() -> Self {
        CallData {
            contract_addr: String::default(),
            data: String::default(),
            cached_size: x_com_lib::rt::CachedSize::new(),
        }
    }
}

impl RequestMessage for CallData {
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.contract_addr.is_empty() {
            my_size += x_com_lib::rt::string_size(1, &self.contract_addr);
        }
        if !self.data.is_empty() {
            my_size += x_com_lib::rt::string_size(2, &self.data);
        }
        self.cached_size.set(my_size as u32);
        my_size
    }

    fn serial_with_output_stream(
        &self,
        os: &mut x_com_lib::CodedOutputStream<'_>,
    ) -> Result<(), Status> {
        if !self.contract_addr.is_empty() {
            os.write_string(1, &self.contract_addr).unwrap();
        }
        if !self.data.is_empty() {
            os.write_string(2, &self.data).unwrap();
        }
        Ok(())
    }

    fn parse_from_input_stream(
        &mut self,
        is: &mut x_com_lib::CodedInputStream<'_>,
    ) -> Result<(), Status> {
        while let Some(tag) = is.read_raw_tag_or_eof().unwrap() {
            match tag {
                10 => {
                    self.contract_addr = is.read_string().map_err(pb_error_to_status)?;
                }
                18 => {
                    self.data = is.read_string().map_err(pb_error_to_status)?;
                }
                _ => {
                    let wire_type = extract_wire_type_from_tag(tag);
                    if wire_type.is_none() {
                        return Err(Status::error("消息格式出错".into()));
                    }
                    let result = is.skip_field(wire_type.unwrap());
                    if let Err(err) = result {
                        return Err(Status::error(err.to_string()));
                    }
                }
            }
        }
        Ok(())
    }
}
