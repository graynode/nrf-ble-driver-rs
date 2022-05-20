use nrf_ble_driver_sys::ffi::*;
use crate::{Adapter, Error, Result};

pub struct L2CapConnectionConfig {
    pub rx_mps: u16,
    pub tx_mps: u16,
    pub rx_queue_size: u8,
    pub tx_queue_size: u8,
    pub ch_count: u8,
}

pub fn set_gatt_connection_config(
    adapter: &mut Adapter,
    connection_tag: u8,
    config: &L2CapConnectionConfig,
) -> Result<()> {
    let l2cap_connection_config = ble_l2cap_conn_cfg_t {
        rx_mps: config.rx_mps,
        tx_mps: config.tx_mps,
        rx_queue_size: config.rx_queue_size,
        tx_queue_size: config.tx_queue_size,
        ch_count: config.ch_count,
    };
    let ble_config = ble_cfg_t {
        conn_cfg: ble_conn_cfg_t {
            conn_cfg_tag: connection_tag,
            params: ble_conn_cfg_t__bindgen_ty_1 {
                l2cap_conn_cfg: l2cap_connection_config,
            },
        },
    };

    unsafe {
        let error_code = sd_ble_cfg_set(
            adapter.get_mut_handle(),
            BLE_CONN_CFGS_BLE_CONN_CFG_L2CAP,
            &ble_config,
            0,
        );
        if error_code == NRF_SUCCESS {
            Ok(())
        } else {
            Err(Error::FFIError(error_code))
        }
    }
}
