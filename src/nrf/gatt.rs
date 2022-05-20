use nrf_ble_driver_sys::ffi::*;
use crate::{Error, Result, Adapter};




pub fn set_gatt_connection_config(
    adapter: &mut Adapter,
    connection_tag: u8,
    att_mtu: u16,
) -> Result<()> {
    let gatt_connection_config = ble_gatt_conn_cfg_t {att_mtu};
    let ble_config = ble_cfg_t {
        conn_cfg: ble_conn_cfg_t {
            conn_cfg_tag: connection_tag,
            params: ble_conn_cfg_t__bindgen_ty_1 {
                gatt_conn_cfg: gatt_connection_config,
            },
        },
    };

    unsafe {
        let error_code = sd_ble_cfg_set(adapter.get_mut_handle(), BLE_CONN_CFGS_BLE_CONN_CFG_GATT, &ble_config, 0);
        if error_code == NRF_SUCCESS {
            Ok(())
        } else {
            Err(Error::FFIError(error_code))
        }
    }
}
