use nrf_sd_api::transport::{serial, h5};
use tokio;
use bytes::{Bytes};


#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyACM0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

#[tokio::main]
async fn main() {
    let mut port = serial::SerialTransport::new(DEFAULT_TTY);
    while let Some(packet_data) = port.next_packet().await {
        let data = packet_data.to_vec();
        println!("{:X?}",  data);
        h5::decode_h5(&data);
    }
}