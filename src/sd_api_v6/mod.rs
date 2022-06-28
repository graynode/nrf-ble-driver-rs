pub mod ble_driver;
pub mod gap;
pub mod ble;
pub mod gatt;
pub mod gattc;
pub mod gatts;


use nrf_ble_driver_sys::ffi;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};




#[derive(Debug)]
pub struct BleDriver {
    adapter: *mut ffi::adapter_t,
    adv_data: Box<ffi::ble_data_t>,
    is_open: bool,
    event_receiver: UnboundedReceiver<EventType>,
    callback_event: UnboundedSender<EventType>,
    is_scanning: bool,
}

#[derive(Debug)]
pub enum EventType {
    Unknown(u32),
    RpcLog(i32, String),
    RpcStatus(i32, String),
    BleCommon(u16),
    BleGap(u16),
    BleGattClient(u16),
    BleGattServer(u16),
    BleL2cap(u16),
}