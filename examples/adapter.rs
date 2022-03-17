use nrf_sd_api::{Adapter};
use tokio;

#[tokio::main]
async fn main() {
    let adapter = Adapter::new("/dev/ttyACM0");

    let mut adapter = adapter.unwrap();
    adapter.open().expect("Error opening port");

    while let Some(event) = adapter.receive_event().await {
        println!("{:?}", event);
    }
}

