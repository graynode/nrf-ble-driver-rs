
use nrf_sd_api::{BleDriver, EventType, gap::*, BluetoothAddress};
use std::{collections::HashMap, hash::Hash};

use tokio;

const EXAMPLE_TAG: u8 = 1;
const DEFAULT_MTU: u16 = 247;

#[derive(Debug, Clone)]
struct DeviceInfo {
    pub name: String,
    pub report_info: GapAdvertisementReport,
}

#[derive(Debug)]
struct AppState {
    pub devices: HashMap<BluetoothAddress, DeviceInfo>,
}

#[tokio::main]
async fn main() {
    let mut app_state = AppState { devices: HashMap::new() };
    //let mut adapter = BleDriver::new("/dev/ttyACM0").unwrap();
    let mut adapter = BleDriver::new("/tmp/ttyV0").unwrap();
    adapter.open().expect("Error opening port");
    adapter.gap_set_role_count_config(&GapConfigRoleCount::default()).unwrap();
    adapter.gatt_set_connection_config(EXAMPLE_TAG, DEFAULT_MTU).unwrap();
    adapter.ble_enable().unwrap();
    adapter.gap_scan_start(&GapScanParameters::default()).unwrap();

    while let Some(event) = adapter.receive_event().await {
        match event {
            EventType::BleGap(gap_event) => handle_gap_event(&mut app_state, &gap_event),
            _ => println!("Unhandled")
        }
        //println!("{:?}", event);
    }
}

fn handle_gap_event(state: &mut AppState, event: &GapEvent) {
    match event {

        GapEvent::AdvertisingReport(report) => {
            
            let name;
            let name_updated;
            
            if let Some(complete_name) = GapAdvertisementReport::find_ad_data(&report, AdvertisingDataType::CompleteLocalName) {
                name = String::from_utf8(complete_name).unwrap();
            } else {
                if let Some(short_name) = GapAdvertisementReport::find_ad_data(&report, AdvertisingDataType::ShortLocalName) {
                    name = String::from_utf8(short_name).unwrap();
                } else {
                    name = String::from("Unknown");
                }
            }
            let address = report.peer_address.address;

            if !state.devices.contains_key(&address) {
                let device_info = DeviceInfo { name: name.clone(), report_info: report.clone() };
                state.devices.insert(address, device_info);
                name_updated = true;
                
            } else if state.devices[&address].name != name {

                let mut device= state.devices[&address].clone();
                device.name = name.clone();
                state.devices.insert(address, device);
                name_updated = true;
            } else {
                name_updated = false;
            }
            
            
            if name_updated {
                let device: &DeviceInfo = &state.devices[&report.peer_address.address];
                println!("{}:{:X?} rssi: {:?} dB, channel: {}",device.name, device.report_info.peer_address.address, device.report_info.rssi, device.report_info.channel_index);
            }
            
            
            /*            let mut a = report.clone();
            if !a.find_ad_data(AdvertisingDataType::CompleteLocalName) {
                a.find_ad_data(AdvertisingDataType::ShortLocalName);
            }
            */
        }
        _ => {}
    }
}
