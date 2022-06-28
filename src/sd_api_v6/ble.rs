use crate::{Error, Result, sd_api_v6::BleDriver};
use nrf_ble_driver_sys::ffi;



impl BleDriver {

    pub fn ble_enable(
        &mut self,
    ) -> Result<()> {
        
        unsafe {
            let mut ram_base: u32 = 0;
            let error_code = ffi::sd_ble_enable(self.adapter, &mut ram_base);
            if error_code == ffi::NRF_SUCCESS {
                Ok(())
            } else {
                Err(Error::FFIError(error_code))
            }
        }
    }
}