use crate::{Error, Result, sd_api_v6::BleDriver};
use nrf_ble_driver_sys::ffi;


impl BleDriver {
    pub fn gattc_set_connection_config(
        &mut self,
        connection_tag: u8,
        write_cmd_tx_queue_size: u8,
    ) -> Result<()> {
        let gattc_connection_config = ffi::ble_gattc_conn_cfg_t { write_cmd_tx_queue_size};
        let ble_config = ffi::ble_cfg_t {
            conn_cfg: ffi::ble_conn_cfg_t {
                conn_cfg_tag: connection_tag,
                params: ffi::ble_conn_cfg_t__bindgen_ty_1 {
                    gattc_conn_cfg: gattc_connection_config,
                },
            },
        };
    
        unsafe {
            let error_code = ffi::sd_ble_cfg_set(self.adapter, ffi::BLE_CONN_CFGS_BLE_CONN_CFG_GATTC, &ble_config, 0);
            if error_code == ffi::NRF_SUCCESS {
                Ok(())
            } else {
                Err(Error::FFIError(error_code))
            }
        }
    }
}