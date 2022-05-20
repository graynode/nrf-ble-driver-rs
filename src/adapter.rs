use super::{nrf::adapter, Error, Result};
use nrf_ble_driver_sys::ffi;
use lazy_static::lazy_static;
use std::collections::hash_map::DefaultHasher;
use std::ptr;
use std::ffi::CStr;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

lazy_static! {
    static ref CALLBACK_SEND: Mutex<Option<UnboundedSender<EventType>>> = Mutex::new(None);
}




#[derive(Debug)]
pub struct Adapter {
    adapter: *mut ffi::adapter_t,
    is_open: bool,
    id: u64,
    rx_channel: UnboundedReceiver<EventType>,
}

#[derive(Debug)]
pub enum EventType {
    RpcLog(i32, String),
    RpcStatus(i32, String),
    BleCommon(u16),
    BleGap(u16),
    BleGattClient(u16),
    BleGattServer(u16),
    BleL2cap(u16),
}

unsafe impl Send for Adapter {}


impl Adapter {
    pub fn new(port_name: &str) -> Result<Adapter> {
        let raw_adapter = adapter::adapter_init(port_name)?;
        let mut hasher = DefaultHasher::new();
        let (send, recv): (UnboundedSender<EventType>, UnboundedReceiver<EventType>) =
            mpsc::unbounded_channel();
        *CALLBACK_SEND.lock().unwrap() = Some(send);
        unsafe {
            (*raw_adapter).internal.hash(&mut hasher);
        }

        Ok(Adapter {
            adapter: raw_adapter,
            is_open: false,
            id: hasher.finish(),
            rx_channel: recv,
        })
    }

    pub fn open(&mut self) -> Result<()> {
        if !self.is_open {
            return adapter::adapter_open(
                self.adapter,
                Some(sd_rpc_status_handler),
                Some(sd_rpc_event_handler),
                Some(sd_rpc_log_handler),
            );
        }

        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        if self.is_open {
            return adapter::adapter_close(self.adapter);
        }

        Ok(())
    }

    pub fn get_handle<'r>(&'r self) -> &'r ffi::adapter_t {
        unsafe { &*self.adapter }
    }

    pub fn get_mut_handle<'r>(&'r mut self) -> &'r mut ffi::adapter_t {
        unsafe { &mut *self.adapter }
    }

    pub async fn receive_event(&mut self) -> Option<EventType> {
        self.rx_channel.recv().await
    }
}

impl Drop for Adapter {
    fn drop(&mut self) {
        let error = adapter::adapter_close(self.adapter);
        match error {
            Ok(_) => {}
            Err(error) => match error {
                Error::FFIError(code) => println!("FFI Error: {}", code),
                _ => {}
            },
        };

        adapter::adapter_delete(self.adapter);
    }
}

extern "C" fn sd_rpc_status_handler(
    adapter: *mut ffi::adapter_t,
    code: ffi::sd_rpc_app_status_t,
    message: *const ::std::os::raw::c_char,
) {
    unsafe {
        let message = CStr::from_ptr(message);
        let lock = CALLBACK_SEND.lock().unwrap();
        if let Some(send) = lock.as_ref().clone() {
            send.send(EventType::RpcStatus(
                code.try_into().unwrap(),
                message.to_string_lossy().into_owned(),
            ));
        }
    }
}

extern "C" fn sd_rpc_event_handler(adapter: *mut ffi::adapter_t, event: *mut ffi::ble_evt_t) {
    let lock = CALLBACK_SEND.lock().unwrap();
    if let Some(send) = lock.as_ref().clone() {
        unsafe {
            send.send(EventType::BleGap((*event).header.evt_id));
        }
    }
}

extern "C" fn sd_rpc_log_handler(
    adapter: *mut ffi::adapter_t,
    severity: ffi::sd_rpc_log_severity_t,
    message: *const ::std::os::raw::c_char,
) {
    unsafe {
        let message = CStr::from_ptr(message);
        let lock = CALLBACK_SEND.lock().unwrap();
        if let Some(send) = lock.as_ref().clone() {
            send.send(EventType::RpcLog(
                severity.try_into().unwrap(),
                message.to_string_lossy().into_owned(),
            ));
        }
    }
}
