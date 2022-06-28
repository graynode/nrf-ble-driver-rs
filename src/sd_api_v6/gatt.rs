use crate::{Error, Result, sd_api_v6::BleDriver};
use nrf_ble_driver_sys::ffi;


impl BleDriver {
    pub fn gatt_set_connection_config(
        &mut self,
        connection_tag: u8,
        att_mtu: u16,
    ) -> Result<()> {
        let gatt_connection_config = ffi::ble_gatt_conn_cfg_t {att_mtu};
        let ble_config = ffi::ble_cfg_t {
            conn_cfg: ffi::ble_conn_cfg_t {
                conn_cfg_tag: connection_tag,
                params: ffi::ble_conn_cfg_t__bindgen_ty_1 {
                    gatt_conn_cfg: gatt_connection_config,
                },
            },
        };
    
        unsafe {
            let error_code = ffi::sd_ble_cfg_set(self.adapter, ffi::BLE_CONN_CFGS_BLE_CONN_CFG_GATT, &ble_config, 0);
            if error_code == ffi::NRF_SUCCESS {
                Ok(())
            } else {
                Err(Error::FFIError(error_code))
            }
        }
    }
}