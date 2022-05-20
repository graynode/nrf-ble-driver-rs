use nrf_ble_driver_sys::ffi::*;
use crate::{Error, Result, Adapter};


pub fn set_gatts_connection_config(
    adapter: &mut Adapter,
    connection_tag: u8,
    hvn_tx_queue_size: u8,
) -> Result<()> {
    let gatts_connection_config = ble_gatts_conn_cfg_t {hvn_tx_queue_size};
    let ble_config = ble_cfg_t {
        conn_cfg: ble_conn_cfg_t {
            conn_cfg_tag: connection_tag,
            params: ble_conn_cfg_t__bindgen_ty_1 {
                gatts_conn_cfg: gatts_connection_config,
            },
        },
    };

    unsafe {
        let error_code = sd_ble_cfg_set(adapter.get_mut_handle(), BLE_CONN_CFGS_BLE_CONN_CFG_GATTS, &ble_config, 0);
        if error_code == NRF_SUCCESS {
            Ok(())
        } else {
            Err(Error::FFIError(error_code))
        }
    }
}