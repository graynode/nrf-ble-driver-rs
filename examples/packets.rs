use nrf_sd_api::transport::{serial, h5};
use tokio::{main, time::timeout};
use std::time::Duration;
use bytes::{Bytes};


#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyACM0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

#[tokio::main]
async fn main() {
    let mut port = serial::SerialTransport::new(DEFAULT_TTY);
    let mut is_running = true;

    println!("{:?}", serial::SerialTransport::available_ports());
    while is_running {
        if let Ok(Some(packet_data)) = timeout(Duration::from_millis(1000), port.next_packet()).await {
            let data = packet_data.to_vec();
            println!("{:X?}",  data);
            h5::decode_h5(&data);
        } else {
            println!("Error");
            is_running = false;
        }
    }


}