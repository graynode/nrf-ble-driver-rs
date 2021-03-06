/* 
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
*/

use crate::{Error, Result};
use nrf_ble_driver_sys::ffi::*;
use std::ffi::{CString, c_void};

const DEFAULT_BAUDRATE: u32 = 1_000_000;

pub fn adapter_init(port_name: &str) -> Result<*mut adapter_t> {
    let port = CString::new(port_name).map_err(Error::NullError)?;
    unsafe {
        let phy = sd_rpc_physical_layer_create_uart(
            port.as_ptr(),
            DEFAULT_BAUDRATE,
            sd_rpc_flow_control_t_SD_RPC_FLOW_CONTROL_NONE,
            sd_rpc_parity_t_SD_RPC_PARITY_NONE,
        );

        if phy.is_null() {
            return Err(Error::InitializationError);
        }

        let link_layer = sd_rpc_data_link_layer_create_bt_three_wire(phy, 250);
        if link_layer.is_null() {
            return Err(Error::InitializationError);
        }

        let transport_layer = sd_rpc_transport_layer_create(link_layer, 1500);
        if transport_layer.is_null() {
            return Err(Error::InitializationError);
        }

        let adapter = sd_rpc_adapter_create(transport_layer);
        if adapter.is_null() {
            return Err(Error::InitializationError);
        }

        Ok(adapter)
    }
}

pub fn adapter_open(adapter: *mut adapter_t,
    status_handler: sd_rpc_status_handler_t,
    event_handler: sd_rpc_evt_handler_t,
    log_handler: sd_rpc_log_handler_t,
    user_data: *mut c_void) -> Result<()> {
    unsafe {
        let error_code = sd_rpc_open(adapter, status_handler, event_handler, log_handler, user_data);
        println!("code: {}", error_code);
        if error_code == NRF_SUCCESS {
            Ok(())
        } else {
            Err(Error::FFIError(error_code))
        }
    }
}

pub fn adapter_close(adapter: *mut adapter_t) -> Result<()> {
    unsafe {
        let error_code = sd_rpc_close(adapter);
        if error_code == NRF_SUCCESS {
            Ok(())
        } else {
            Err(Error::FFIError(error_code))
        }
    }
}

pub fn adapter_delete(adapter: *mut adapter_t) {
    unsafe {
        sd_rpc_adapter_delete(adapter);
    }
}