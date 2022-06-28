use crate::{sd_api_v6::BleDriver, Error, Result};
use nrf_ble_driver_sys::ffi;
use std::ptr;

#[derive(Debug)]
pub enum BleGapEvent {
    Connected,
}

pub struct GapConfigRoleCount {
    /// Maximum number of advertising sets. Default value is 1.
    pub advertising_set_count: u8,
    /// Maximum number of connections concurrently acting as a peripheral. Default value is 1.
    pub peripheral_role_count: u8,
    /// Maximum number of connections concurrently acting as a central. Default value is 3.
    pub central_role_count: u8,
    pub central_security_count: u8,
    pub qos_channel_survey_role_available: u8,
}

impl GapConfigRoleCount {
    pub fn new(
        advertising_set_count: u8,
        peripheral_role_count: u8,
        central_role_count: u8,
        central_security_count: u8,
        qos_channel_survey_role_available: u8,
    ) -> GapConfigRoleCount {
        GapConfigRoleCount {
            advertising_set_count,
            peripheral_role_count,
            central_role_count,
            central_security_count,
            qos_channel_survey_role_available,
        }
    }
}

impl Default for GapConfigRoleCount {
    fn default() -> Self {
        GapConfigRoleCount {
            advertising_set_count: 1,
            peripheral_role_count: 1,
            central_role_count: 3,
            central_security_count: 1,
            qos_channel_survey_role_available: 0,
        }
    }
}

pub struct GapScanParameters {
    /// If 1, the scanner will accept extended advertising packets.
    /// If set to 0, the scanner will not receive advertising packets
    /// on secondary advertising channels, and will not be able
    /// to receive long advertising PDUs.
    pub extended: u8,
    pub report_incomplete_events: u8,
    pub active: u8,
    pub filter_policy: u8,
    pub scan_phys: u8,
    pub interval: u16,
    pub window: u16,
    pub timeout: u16,
    pub channel_mask: [u8; 5usize],
}

impl GapScanParameters {
    pub fn new(
        extended: u8,
        report_incomplete_events: u8,
        active: u8,
        filter_policy: u8,
        scan_phys: u8,
        interval: u16,
        window: u16,
        timeout: u16,
        channel_mask: [u8; 5usize],
    ) -> GapScanParameters {
        GapScanParameters {
            extended,
            report_incomplete_events,
            active,
            filter_policy,
            scan_phys,
            interval,
            window,
            timeout,
            channel_mask,
        }
    }
}

impl Default for GapScanParameters {
    fn default() -> Self {
        GapScanParameters {
            extended: 1,
            report_incomplete_events: 0,
            active: 0,
            filter_policy: 0,
            scan_phys: 1,
            interval: 0xa0,
            window: 0x50,
            timeout: 0,
            channel_mask: [0; 5],
        }
    }
}

impl BleDriver {
    pub fn gap_set_connection_config(
        &mut self,
        connection_tag: u8,
        connection_count: u8,
        event_length: u16,
    ) -> Result<()> {
        let gap_connection_config = ffi::ble_gap_conn_cfg_t {
            conn_count: connection_count,
            event_length,
        };
        let ble_config = ffi::ble_cfg_t {
            conn_cfg: ffi::ble_conn_cfg_t {
                conn_cfg_tag: connection_tag,
                params: ffi::ble_conn_cfg_t__bindgen_ty_1 {
                    gap_conn_cfg: gap_connection_config,
                },
            },
        };

        unsafe {
            let error_code = ffi::sd_ble_cfg_set(
                self.adapter,
                ffi::BLE_CONN_CFGS_BLE_CONN_CFG_GAP,
                &ble_config,
                0,
            );
            if error_code == ffi::NRF_SUCCESS {
                Ok(())
            } else {
                Err(Error::FFIError(error_code))
            }
        }
    }

    pub fn gap_set_role_count_config(&mut self, config: &GapConfigRoleCount) -> Result<()> {
        let ble_config = ffi::ble_cfg_t {
            gap_cfg: ffi::ble_gap_cfg_t {
                role_count_cfg: ffi::ble_gap_cfg_role_count_t {
                    adv_set_count: config.advertising_set_count,
                    periph_role_count: config.peripheral_role_count,
                    central_role_count: config.central_role_count,
                    central_sec_count: config.central_security_count,
                    _bitfield_align_1: [0; 0],
                    _bitfield_1: ffi::ble_gap_cfg_role_count_t::new_bitfield_1(
                        config.qos_channel_survey_role_available,
                    ),
                },
            },
        };

        unsafe {
            let error_code = ffi::sd_ble_cfg_set(
                self.adapter,
                ffi::BLE_GAP_CFGS_BLE_GAP_CFG_ROLE_COUNT,
                &ble_config,
                0,
            );
            if error_code == ffi::NRF_SUCCESS {
                Ok(())
            } else {
                Err(Error::FFIError(error_code))
            }
        }
    }

    pub fn gap_scan_start(&mut self, scan_parameters: &GapScanParameters) -> Result<()> {
        let mut error_code = ffi::NRF_SUCCESS;

        if self.is_scanning {
            let scan_params: *const ffi::ble_gap_scan_params_t = ptr::null();
            unsafe {
                error_code =
                    ffi::sd_ble_gap_scan_start(self.adapter, scan_params, &*self.adv_data);
            }
        } else {
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
                error_code =
                    ffi::sd_ble_gap_scan_start(self.adapter, &scan_params, &*self.adv_data);
            }
        }

        if error_code == ffi::NRF_SUCCESS {
            self.is_scanning = true;
            Ok(())
        } else {
            Err(Error::FFIError(error_code))
        }
    }

    pub fn into_gap_event(&mut self, event_id: u32, gap_event: ffi::ble_gap_evt_t) {
        println!("GAP Event: {}", event_id);
        match event_id {
            ffi::BLE_GAP_EVTS_BLE_GAP_EVT_ADV_REPORT => unsafe {
                println!("Address: {:?}", gap_event.params.adv_report.peer_addr.addr);
                if self.is_scanning {
                    self.gap_scan_start(&GapScanParameters::default()).unwrap();
                }
            },
            _ => println!("Unknown"),
        }
    }
}
