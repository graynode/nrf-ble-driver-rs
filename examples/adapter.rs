use nrf_sd_api::{BleDriver, gap::*};

use tokio;

const EXAMPLE_TAG: u8 = 1;
const DEFAULT_MTU: u16 = 247;

#[tokio::main]
async fn main() {
    let mut adapter = BleDriver::new("/dev/ttyACM0").unwrap();
    adapter.open().expect("Error opening port");
    adapter.gap_set_role_count_config(&GapConfigRoleCount::default()).unwrap();
    adapter.gatt_set_connection_config(EXAMPLE_TAG, DEFAULT_MTU).unwrap();
    adapter.ble_enable().unwrap();
    adapter.gap_scan_start(&GapScanParameters::default()).unwrap();

    while let Some(event) = adapter.receive_event().await {
        println!("{:?}", event);
    }
}

