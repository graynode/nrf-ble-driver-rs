use nrf_sd_api::{Adapter, ble, gap::{self, GapScanParameters}, gatt};

use tokio;

#[tokio::main]
async fn main() {
    let mut adapter = Adapter::new("/dev/ttyACM0").unwrap();

    adapter.open().expect("Error opening port");
    let gap_role_count_config = gap::GapConfigRoleCount::new(1, 0, 1, 0, 0);
    gap::set_role_count_config(&mut adapter, &gap_role_count_config).unwrap();
    gatt::set_gatt_connection_config(&mut adapter, 1, 247).unwrap();
    ble::ble_enable(&mut adapter).unwrap();

    let scan_params = GapScanParameters::new(1, 0, 0, 0, 1, 0xa0, 0x50, 0, [0;5]);
    gap::scan_start(&mut adapter, &scan_params).unwrap();

    while let Some(event) = adapter.receive_event().await {
        println!("{:?}", event);
    }
}

