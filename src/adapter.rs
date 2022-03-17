


use super::{Error, Result, api};
use std::sync::Mutex;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::mpsc;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use lazy_static::lazy_static;
use std::ffi::{CStr};




lazy_static! {
    static ref CALLBACK_SEND: Mutex<Option<UnboundedSender<EventType>>> = Mutex::new(None);
}

#[derive(Debug)]
pub struct Adapter {
    adapter: *mut api::adapter_t,
    is_open: bool,
    id: u64,
    rx_channel: UnboundedReceiver<EventType>,
}


#[derive(Debug)]
pub enum EventType {
    RpcLog(u32, String),
    RpcStatus(u32, String),
    BleCommon(u16),
    BleGap(u16),
    BleGattClient(u16),
    BleGattServer(u16),
    BleL2cap(u16),
}

unsafe impl Send for Adapter {}


impl Adapter {
    pub fn new(port_name: &str) -> Result<Adapter> {
        let raw_adapter = api::adapter_init(port_name)?;
        let mut hasher = DefaultHasher::new();
        let (send, recv): (UnboundedSender<EventType>, UnboundedReceiver<EventType>) = mpsc::unbounded_channel();
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
            let error_code = api::adapter_open(self.adapter,
            Some(sd_rpc_status_handler),
        Some(sd_rpc_event_handler),
    Some(sd_rpc_log_handler));
            if error_code != api::NRF_SUCCESS {
                return Err(Error::FFIError(error_code));
            }
        }

        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        if self.is_open {
            let error = api::adapter_close(self.adapter);
            if error != api::NRF_SUCCESS {
                println!("error in close {}", error);
                return Err(Error::FFIError(error));
            }
        }

        Ok(())
    }

    pub async fn receive_event(&mut self) -> Option<EventType> {
        self.rx_channel.recv().await
    }
}


impl Drop for Adapter {
    fn drop(&mut self) {
        println!("Adapter::Drop");

        let error = api::adapter_close(self.adapter);
        if error != api::NRF_SUCCESS {
            println!("error: {}", error);
        }

        api::adapter_delete(self.adapter);
    }
}




extern "C" fn sd_rpc_status_handler(
    adapter: *mut api::adapter_t,
    code: api::sd_rpc_app_status_t,
    message: *const ::std::os::raw::c_char,
) {
    unsafe {
        let message = CStr::from_ptr(message);
        let lock = CALLBACK_SEND.lock().unwrap();
        if let Some(send) = lock.as_ref().clone() {
            send.send(EventType::RpcStatus(code, message.to_string_lossy().into_owned()));
        }
    }
}

extern "C" fn sd_rpc_event_handler(adapter: *mut api::adapter_t, event: *mut api::ble_evt_t) {
    let lock = CALLBACK_SEND.lock().unwrap();
    if let Some(send) = lock.as_ref().clone() {
        unsafe {
            send.send(EventType::BleGap((*event).header.evt_id));
        }
    }
}

extern "C" fn sd_rpc_log_handler(
    adapter: *mut api::adapter_t,
    severity: api::sd_rpc_log_severity_t,
    message: *const ::std::os::raw::c_char,
) { 
    unsafe {
        let message = CStr::from_ptr(message);
        let lock = CALLBACK_SEND.lock().unwrap();
        if let Some(send) = lock.as_ref().clone() {
            send.send(EventType::RpcLog(severity, message.to_string_lossy().into_owned()));
        }
    }
}
