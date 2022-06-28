use crate::{Error, Result, sd_api_v6::*};
use nrf_ble_driver_sys::ffi;
use std::ffi::{c_void, CStr, CString};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};


const DEFAULT_BAUDRATE: u32 = 1_000_000;



unsafe impl Send for BleDriver {}



impl BleDriver {
    pub fn new(port_name: &str) -> Result<BleDriver> {
        let raw_adapter = BleDriver::adapter_init(port_name)?;
        // Create our single boxed advertising data buffer
        let mut p_data = vec![0; ffi::BLE_GAP_SCAN_BUFFER_EXTENDED_MAX as usize].into_boxed_slice();
        let adv_data = Box::new(ffi::ble_data_t {
            p_data: p_data.as_mut_ptr(),
            len: p_data.len() as u16,
        });
        std::mem::forget(p_data);
        let (send, recv): (UnboundedSender<EventType>, UnboundedReceiver<EventType>) =
            mpsc::unbounded_channel();

        Ok(BleDriver {
            adapter: raw_adapter,
            adv_data,
            is_open: false,
            event_receiver: recv,
            callback_event: send,
            is_scanning: false,
        })
    }

    pub fn open(&mut self) -> Result<()> {
        if !self.is_open {
            unsafe {
                let error_code = ffi::sd_rpc_open(
                    self.adapter,
                    Some(sd_rpc_status_handler),
                    Some(sd_rpc_event_handler),
                    Some(sd_rpc_log_handler),
                    self as *mut _ as *mut c_void,
                );
                if error_code == ffi::NRF_SUCCESS {
                    return Ok(());
                } else {
                    return Err(Error::FFIError(error_code));
                }
            }
        }

        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        if self.is_open {
            unsafe {
                let error_code = ffi::sd_rpc_close(self.adapter);
                if error_code == ffi::NRF_SUCCESS {
                    return Ok(());
                } else {
                    return Err(Error::FFIError(error_code));
                }
            }
        }

        Ok(())
    }

    pub fn on_ffi_callback(&mut self, event: EventType) {
        let _result = self.callback_event.send(event);
    }

    pub async fn receive_event(&mut self) -> Option<EventType> {
        self.event_receiver.recv().await
    }

    fn adapter_init(port_name: &str) -> Result<*mut ffi::adapter_t> {
        let port = CString::new(port_name).map_err(Error::NullError)?;
        unsafe {
            let phy = ffi::sd_rpc_physical_layer_create_uart(
                port.as_ptr(),
                DEFAULT_BAUDRATE,
                ffi::sd_rpc_flow_control_t_SD_RPC_FLOW_CONTROL_NONE,
                ffi::sd_rpc_parity_t_SD_RPC_PARITY_NONE,
            );

            if phy.is_null() {
                return Err(Error::InitializationError);
            }

            let link_layer = ffi::sd_rpc_data_link_layer_create_bt_three_wire(phy, 250);
            if link_layer.is_null() {
                return Err(Error::InitializationError);
            }

            let transport_layer = ffi::sd_rpc_transport_layer_create(link_layer, 1500);
            if transport_layer.is_null() {
                return Err(Error::InitializationError);
            }

            let adapter = ffi::sd_rpc_adapter_create(transport_layer);
            if adapter.is_null() {
                return Err(Error::InitializationError);
            }

            Ok(adapter)
        }
    }

    

}

impl Drop for BleDriver {
    fn drop(&mut self) {
        match self.close() {
            Ok(()) => {}
            Err(e) => println!("{:?}", e),
        }
        unsafe {
            ffi::sd_rpc_adapter_delete(self.adapter);
        }
    }
}

extern "C" fn sd_rpc_status_handler(
    adapter: *mut ffi::adapter_t,
    code: ffi::sd_rpc_app_status_t,
    message: *const ::std::os::raw::c_char,
) {
    unsafe {
        if !(*adapter).user_data.is_null() {
            let message = CStr::from_ptr(message);
            let user_data: &mut BleDriver = &mut *((*adapter).user_data as *mut BleDriver);
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
            let user_data: &mut BleDriver = &mut *((*adapter).user_data as *mut BleDriver);

            match event_id {
                ffi::BLE_EVT_INVALID => println!("Invalid event"),
                ffi::BLE_EVT_BASE..=ffi::BLE_EVT_LAST => println!("BLE Common Event: {}", event_id),
                ffi::BLE_GAP_EVT_BASE..=ffi::BLE_GAP_EVT_LAST => {
                    user_data.into_gap_event(event_id, (*rpc_event).evt.gap_evt);
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
    /*
    unsafe {
        if !(*adapter).user_data.is_null() {
            let message = CStr::from_ptr(message);
            let user_data: &mut BleDriver = &mut *((*adapter).user_data as *mut BleDriver);
            user_data.on_ffi_callback(EventType::RpcLog(
                severity,
                message.to_string_lossy().to_string(),
            ));
        }
    }
    */
}
