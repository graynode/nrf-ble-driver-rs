pub mod ble_driver;
pub mod gap;
pub mod ble;
pub mod gatt;
pub mod gattc;
pub mod gatts;


use nrf_ble_driver_sys::ffi;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use self::gap::GapEvent;


pub type BluetoothAddress = [u8; 6];


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
    RpcLog(i32, String),
    RpcStatus(i32, String),
    BleCommon(u32),
    BleGap(GapEvent),
    BleGattClient(u32),
    BleGattServer(u32),
    BleL2cap(u32),
    Unknown(u32),
    Invalid,
}