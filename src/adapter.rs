use crate::gap;

use super::{nrf::adapter, Error, Result};
use nrf_ble_driver_sys::ffi;
use std::ffi::{c_void, CStr};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Debug)]
pub struct Adapter {
    adapter: *mut ffi::adapter_t,
    is_open: bool,
    event_receiver: UnboundedReceiver<EventType>,
    callback_event: UnboundedSender<EventType>,
}

#[derive(Debug)]
pub enum EventType {
    Unknown(u32),
    RpcLog(i32, String),
    RpcStatus(i32, String),
    BleCommon(u16),
    BleGap(gap::BleGapEvent),
    BleGattClient(u16),
    BleGattServer(u16),
    BleL2cap(u16),
}

unsafe impl Send for Adapter {}

impl Adapter {
    pub fn new(port_name: &str) -> Result<Adapter> {
        let raw_adapter = adapter::adapter_init(port_name)?;
        let (send, recv): (UnboundedSender<EventType>, UnboundedReceiver<EventType>) =
            mpsc::unbounded_channel();

        Ok(Adapter {
            adapter: raw_adapter,
            is_open: false,
            event_receiver: recv,
            callback_event: send,
        })
    }

    pub fn open(&mut self) -> Result<()> {
        if !self.is_open {
            return adapter::adapter_open(
                self.adapter,
                Some(sd_rpc_status_handler),
                Some(sd_rpc_event_handler),
                Some(sd_rpc_log_handler),
                self as *mut _ as *mut c_void,
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

    pub fn on_ffi_callback(&mut self, event: EventType) {
        let _result = self.callback_event.send(event);
    }

    pub fn get_handle<'r>(&'r self) -> &'r ffi::adapter_t {
        unsafe { &*self.adapter }
    }

    pub fn get_mut_handle<'r>(&'r mut self) -> &'r mut ffi::adapter_t {
        unsafe { &mut *self.adapter }
    }

    pub async fn receive_event(&mut self) -> Option<EventType> {
        self.event_receiver.recv().await
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

pub trait GapApi {
    fn gap_connection_config(&self);
    fn gap_scan_start(&self, scan_parameters: &gap::GapScanParameters) -> Result<()>;
}

extern "C" fn sd_rpc_status_handler(
    adapter: *mut ffi::adapter_t,
    code: ffi::sd_rpc_app_status_t,
    message: *const ::std::os::raw::c_char,
) {
    unsafe {
        if !(*adapter).user_data.is_null() {
            let message = CStr::from_ptr(message);
            let user_data: &mut Adapter = &mut *((*adapter).user_data as *mut Adapter);
            user_data.on_ffi_callback(EventType::RpcStatus(
                code,
                message.to_string_lossy().into_owned(),
            ));
        }
    }
}

extern "C" fn sd_rpc_event_handler(adapter: *mut ffi::adapter_t, rpc_event: *mut ffi::ble_evt_t) {
    let mut event = EventType::Unknown(0);

    unsafe {
        let event_id: u32 = (*rpc_event).header.evt_id.into();
        if !(*adapter).user_data.is_null() {
            let user_data: &mut Adapter = &mut *((*adapter).user_data as *mut Adapter);

            match event_id {
                ffi::BLE_EVT_INVALID => println!("Invalid event"),
                ffi::BLE_EVT_BASE..=ffi::BLE_EVT_LAST => println!("BLE Common Event: {}", event_id),
                ffi::BLE_GAP_EVT_BASE..=ffi::BLE_GAP_EVT_LAST => {
                    gap::into_gap_event(event_id, (*rpc_event).evt.gap_evt)
                }
                ffi::BLE_GATTC_EVT_BASE..=ffi::BLE_GATTC_EVT_LAST => {
                    println!("GATTC Event: {}", event_id)
                }
                ffi::BLE_GATTS_EVT_BASE..=ffi::BLE_GATTS_EVT_LAST => {
                    println!("GATTS Event: {}", event_id)
                }
                ffi::BLE_L2CAP_EVT_BASE..=ffi::BLE_L2CAP_EVT_LAST => {
                    println!("L2CAP Event: {}", event_id)
                }
                _ => event = EventType::Unknown(event_id),
            }
            user_data.on_ffi_callback(event);
        }
    }
}

extern "C" fn sd_rpc_log_handler(
    adapter: *mut ffi::adapter_t,
    severity: ffi::sd_rpc_log_severity_t,
    message: *const ::std::os::raw::c_char,
) {
    unsafe {
        if !(*adapter).user_data.is_null() {
            let message = CStr::from_ptr(message);
            let user_data: &mut Adapter = &mut *((*adapter).user_data as *mut Adapter);
            user_data.on_ffi_callback(EventType::RpcLog(
                severity,
                message.to_string_lossy().to_string(),
            ));
        }
    }
}


impl GapApi for Adapter {
    fn gap_connection_config(&self) {
        println!("gap donkey");
    }

    fn gap_scan_start(&self, scan_parameters: &gap::GapScanParameters) -> Result<()> {
        let mut p_data = vec![0; ffi::BLE_GAP_SCAN_BUFFER_EXTENDED_MAX as usize].into_boxed_slice();
        let adv_data = Box::new(ffi::ble_data_t {
            p_data: p_data.as_mut_ptr(),
            len: p_data.len() as u16,
        });
        std::mem::forget(p_data);

        let scan_params = ffi::ble_gap_scan_params_t {
            _bitfield_align_1: [0; 0],
            _bitfield_1: ffi::ble_gap_scan_params_t::new_bitfield_1(
                scan_parameters.extended,
                scan_parameters.report_incomplete_events,
                scan_parameters.active,
                scan_parameters.filter_policy,
            ),
            scan_phys: scan_parameters.scan_phys,
            interval: scan_parameters.interval,
            window: scan_parameters.window,
            timeout: scan_parameters.timeout,
            channel_mask: scan_parameters.channel_mask,
        };

        unsafe {
            let error_code = ffi::sd_ble_gap_scan_start(self.adapter, &scan_params, &*adv_data);

            if error_code == ffi::NRF_SUCCESS {
                Ok(())
            } else {
                Err(Error::FFIError(error_code))
            }
        }
    }
}