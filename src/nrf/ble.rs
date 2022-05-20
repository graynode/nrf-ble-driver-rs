
use nrf_ble_driver_sys::ffi::*;
use crate::{Error, Result, Adapter};


pub fn ble_enable(
    adapter: &mut Adapter,
) -> Result<()> {
    
    unsafe {
        let mut ram_base: u32 = 0;
        let error_code = sd_ble_enable(adapter.get_mut_handle(), &mut ram_base);
        if error_code == NRF_SUCCESS {
            Ok(())
        } else {
            Err(Error::FFIError(error_code))
        }
    }
}

pub fn set_ble_common_config() {}

pub fn set_ble_gap_config() {}

pub fn set_gatts_config() {}
