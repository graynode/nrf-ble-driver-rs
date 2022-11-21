use crate::{sd_api_v6::BleDriver, Error, Result, BluetoothAddress};
use nrf_ble_driver_sys::ffi;
use std::{ptr, slice, result, str, clone};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;


#[derive(Debug)]
pub enum GapEvent {
    Connect,
    Disconnect,
    ConnectionParametersUpdate,
    SecurityParametersRequest,
    SecurityInformationRequest,
    PassKeyDisplay,
    KeyPress,
    AuthenticationKeyRequest,
    LESecureKeyDiffieHellmanKeyRequest,
    AuthenticationStatus,
    ConnectionSecurityUpdate,
    Timeout,
    RSSIChanged,
    AdvertisingReport(GapAdvertisementReport),
    SecurityRequest,
    ConnectionParameterUpdateRequest,
    ScanRequestReport,
    PhyUpdateRequest,
    PhyUpdate,
    DataLengthUpdateRequest,
    DataLengthUpdate,
    QOSChannelSurveyReport,
    AdvertisingSetTerminated,
    Unknown(u32),
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
            active: 1,
            filter_policy: ffi::BLE_GAP_SCAN_FP_ACCEPT_ALL as u8,
            scan_phys: ffi::BLE_GAP_PHY_AUTO as u8,
            interval: 0xa0,
            window: 50,
            timeout: 0,
            channel_mask: [0; 5],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GapAddressType {
    Public,
    RandomStatic,
    PrivateResolvable,
    PrivateNonResolvable,
    Anonymous,
    Unknown(u8),
}

#[derive(Debug, Clone, Copy)]
pub struct GapAddress {
    pub address_id_peer: bool,
    pub address_type: GapAddressType,
    pub address: BluetoothAddress,
}

#[derive(Debug, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum GapPhy {
    Auto = ffi::BLE_GAP_PHY_AUTO,
    OneMbps = ffi::BLE_GAP_PHY_1MBPS,
    TwoMbps = ffi::BLE_GAP_PHY_2MBPS,
    Coded = ffi::BLE_GAP_PHY_CODED,
    NotConfigured = ffi::BLE_GAP_PHY_NOT_SET,
    #[num_enum(default)]
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum TxPowerLevel {
    Value(i8),
    Invalid,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum GapSetId {
    Value(u8),
    NotAvailable,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum AdvertisingDataType {
    Flags = ffi::BLE_GAP_AD_TYPE_FLAGS as u8,
    ServiceUUIDIncomplete16Bit = ffi::BLE_GAP_AD_TYPE_16BIT_SERVICE_UUID_MORE_AVAILABLE as u8,
    ServiceUUIDComplete16Bit = ffi::BLE_GAP_AD_TYPE_16BIT_SERVICE_UUID_COMPLETE as u8,
    ServiceUUIDIncomplete32Bit = ffi::BLE_GAP_AD_TYPE_32BIT_SERVICE_UUID_MORE_AVAILABLE as u8,
    ServiceUUIDComplete32Bit = ffi::BLE_GAP_AD_TYPE_32BIT_SERVICE_UUID_COMPLETE as u8,
    ServiceUUIDIncomplete128Bit = ffi::BLE_GAP_AD_TYPE_128BIT_SERVICE_UUID_MORE_AVAILABLE as u8,
    ServiceUUIDComplete128Bit = ffi::BLE_GAP_AD_TYPE_128BIT_SERVICE_UUID_COMPLETE as u8,
    ShortLocalName = ffi::BLE_GAP_AD_TYPE_SHORT_LOCAL_NAME as u8,
    CompleteLocalName = ffi::BLE_GAP_AD_TYPE_COMPLETE_LOCAL_NAME as u8,
    TxPowerLevel = ffi::BLE_GAP_AD_TYPE_TX_POWER_LEVEL as u8,
    ClassOfDevice = ffi::BLE_GAP_AD_TYPE_CLASS_OF_DEVICE as u8,
    SimplePairingHashC = ffi::BLE_GAP_AD_TYPE_SIMPLE_PAIRING_HASH_C as u8,
    SimpleRandomizerR = ffi::BLE_GAP_AD_TYPE_SIMPLE_PAIRING_RANDOMIZER_R as u8,
    SecurityManagerTKValue = ffi::BLE_GAP_AD_TYPE_SECURITY_MANAGER_TK_VALUE as u8,
    SecurityManagerOOBFlags = ffi::BLE_GAP_AD_TYPE_SECURITY_MANAGER_OOB_FLAGS as u8,
    PeripheralConnectionIntervalRange = ffi::BLE_GAP_AD_TYPE_SLAVE_CONNECTION_INTERVAL_RANGE as u8,
    SolicitedServiceUUIDS16Bit = ffi::BLE_GAP_AD_TYPE_SOLICITED_SERVICE_UUIDS_16BIT as u8,
    SolicitedServiceUUIDS128Bit = ffi::BLE_GAP_AD_TYPE_SOLICITED_SERVICE_UUIDS_128BIT as u8,
    ServiceData = ffi::BLE_GAP_AD_TYPE_SERVICE_DATA as u8,
    PublicTargetAddress = ffi::BLE_GAP_AD_TYPE_PUBLIC_TARGET_ADDRESS as u8,
    RandomTargetAddress = ffi::BLE_GAP_AD_TYPE_RANDOM_TARGET_ADDRESS as u8,
    Appearance = ffi::BLE_GAP_AD_TYPE_APPEARANCE as u8,
    AdvertisingInterval = ffi::BLE_GAP_AD_TYPE_ADVERTISING_INTERVAL as u8,
    LEBluetoothDeviceAddress = ffi::BLE_GAP_AD_TYPE_LE_BLUETOOTH_DEVICE_ADDRESS as u8,
    LERole = ffi::BLE_GAP_AD_TYPE_LE_ROLE as u8,
    SimplePairingHashC256 = ffi::BLE_GAP_AD_TYPE_SIMPLE_PAIRING_HASH_C256 as u8,
    SimplePairingRandomizerR256 = ffi::BLE_GAP_AD_TYPE_SIMPLE_PAIRING_RANDOMIZER_R256 as u8,
    ServiceData32BitUUID = ffi::BLE_GAP_AD_TYPE_SERVICE_DATA_32BIT_UUID as u8,
    ServiceData128BitUUID = ffi::BLE_GAP_AD_TYPE_SERVICE_DATA_128BIT_UUID as u8,
    LESecureConnectionConfirmationValue = ffi::BLE_GAP_AD_TYPE_LESC_CONFIRMATION_VALUE as u8,
    LESecureConnectionRandomValue = ffi::BLE_GAP_AD_TYPE_LESC_RANDOM_VALUE as u8,
    URI = ffi::BLE_GAP_AD_TYPE_URI as u8,
    InformationData3D = ffi::BLE_GAP_AD_TYPE_3D_INFORMATION_DATA as u8,
    ManufacturerSpecificData = ffi::BLE_GAP_AD_TYPE_MANUFACTURER_SPECIFIC_DATA as u8,
}



#[derive(Debug, Clone)]
pub struct GapAdvertisementReport {
    //pub report type
    pub peer_address: GapAddress,
    pub direct_address: GapAddress,
    pub primary_phy: GapPhy,
    pub secondary_phy: GapPhy,
    pub tx_power: TxPowerLevel,
    pub rssi: i8,
    pub channel_index: u8,
    pub set_id: GapSetId,
    pub data: Vec<u8>,
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
        let error_code;

        if self.is_scanning {
            let scan_params: *const ffi::ble_gap_scan_params_t = ptr::null();
            unsafe {
                error_code = ffi::sd_ble_gap_scan_start(self.adapter, scan_params, &*self.adv_data);
            }
        } else {
            let scan_params = ffi::ble_gap_scan_params_t {
                _bitfield_align_1: [0; 0],
                _bitfield_1: ffi::ble_gap_scan_params_t::new_bitfield_1(
                    scan_parameters.extended,
                    0, // not supported in this softdevice
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

    pub fn handle_gap_event(&mut self, event_id: u32, gap_event: &ffi::ble_gap_evt_t) -> GapEvent {
        unsafe {
            let event = match event_id {
                ffi::BLE_GAP_EVTS_BLE_GAP_EVT_ADV_REPORT => {
                    if self.is_scanning {
                        self.gap_scan_start(&GapScanParameters::default()).unwrap();
                    }
                    GapEvent::AdvertisingReport(GapAdvertisementReport::from_ffi(&gap_event.params.adv_report))
                }
                id => GapEvent::Unknown(id)

            };

            event
        }
    }

    
}

impl GapAdvertisementReport {

    pub fn find_ad_data(advertisement: &GapAdvertisementReport, adtype: AdvertisingDataType) -> Option<Vec<u8>> {
        let mut index = 0;
        //println!("{:X?}", advertisement.data);

        while index < advertisement.data.len() {
            let length = advertisement.data[index] as usize;
            let ad_type = advertisement.data[index + 1];

            //println!("length: {}, index: {}", length, index);

            if ad_type == adtype as u8 {
                return Some(advertisement.data[index + 2..index + length + 1].to_vec());
            }

            index += length + 1;
        }

        None
    }

    fn from_ffi(adv_report: &ffi::ble_gap_evt_adv_report_t) -> GapAdvertisementReport {
        //let primary_phy = GapPhy::try_from(adv_report.primary_phy as u32);
        
        let tx_power = if adv_report.tx_power == ffi::BLE_GAP_POWER_LEVEL_INVALID as i8 {
            TxPowerLevel::Invalid
        } else {
            TxPowerLevel::Value(adv_report.tx_power)
        };
           
        let set_id = match adv_report.set_id as u32{
            ffi::BLE_GAP_ADV_REPORT_SET_ID_NOT_AVAILABLE => GapSetId::NotAvailable,
            id => GapSetId::Value(id as u8),
        };

        let data;
        unsafe {
            data = slice::from_raw_parts(adv_report.data.p_data, adv_report.data.len as usize).to_vec();
        }
    
        GapAdvertisementReport {
            peer_address: GapAddress::from(&adv_report.peer_addr),
            direct_address: GapAddress::from(&adv_report.direct_addr),
            primary_phy: GapPhy::try_from(adv_report.primary_phy as u32).unwrap(),
            secondary_phy: GapPhy::try_from(adv_report.secondary_phy as u32).unwrap(),
            tx_power,
            rssi: adv_report.rssi,
            channel_index: adv_report.ch_index,
            set_id,
            data
        }

    }
}


impl GapAddress {
    fn from(gap_address: &ffi::ble_gap_addr_t) -> GapAddress {
        let address_type = match gap_address.addr_type() as u32 {
            ffi::BLE_GAP_ADDR_TYPE_PUBLIC => GapAddressType::Public,
            ffi::BLE_GAP_ADDR_TYPE_RANDOM_STATIC => GapAddressType::RandomStatic,
            ffi::BLE_GAP_ADDR_TYPE_RANDOM_PRIVATE_RESOLVABLE => GapAddressType::PrivateResolvable,
            ffi::BLE_GAP_ADDR_TYPE_RANDOM_PRIVATE_NON_RESOLVABLE => {
                GapAddressType::PrivateNonResolvable
            }
            ffi::BLE_GAP_ADDR_TYPE_ANONYMOUS => GapAddressType::Anonymous,
            unknown => GapAddressType::Unknown(unknown as u8),
        };

        GapAddress {
            address_id_peer: gap_address.addr_id_peer() != 0,
            address_type,
            address: gap_address.addr,
        }
    }
}

fn check_name(data: &ffi::ble_data_t) {
    unsafe {
        let safe_data = slice::from_raw_parts(data.p_data, data.len as usize).to_vec();
        println!("{:X?}", safe_data);
    }
}