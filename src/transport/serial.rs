
use crate::transport::slip;
use crate::{Error, Result};

use futures::stream::StreamExt;
use tokio_serial::{SerialPortBuilder, SerialStream, SerialPortBuilderExt, SerialPortType};
use tokio_util::codec::{Encoder, Decoder, Framed};
use bytes::{BytesMut, Bytes, BufMut, Buf};
use std::{io, env};


use super::slip::SLIP_END;

const DEFAULT_BAUDRATE: u32 = 1_000_000;
#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyACM0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

const NORDIC_USB_VID: u16 = 0x1915;
const NORDIC_USB_PID: u16 = 0xC00A;

struct PacketCodec;

pub struct SerialTransport {
    codec: Framed<SerialStream, PacketCodec>,
    //serial_port: SerialPortBuilder,
    //serial_stream: &SerialStream,
}


impl SerialTransport {
    
    pub fn new(port: &str) -> Self {
        let port = tokio_serial::new(port, DEFAULT_BAUDRATE);
        let stream = port.open_native_async().unwrap();
        let reader = PacketCodec.framed(stream);
        /*
        while let Some(packet_result) = reader.next().await {

        }
        */
        SerialTransport { codec: reader }
        //SerialTransport { serial_port: port, serial_stream: stream }
    }

    pub fn open(&mut self) {
        
    }

    pub async fn next_packet(&mut self) -> Option<Bytes> {
        if let Some(packet_result) = self.codec.next().await {
            Some(packet_result.unwrap())
        } else {
            None
        }
    }

    pub fn available_ports() -> Vec<String> {
        let mut result = Vec::new();
        if let Ok(ports) = tokio_serial::available_ports()  {
            for port in ports {
                match (port.port_type) {
                    SerialPortType::UsbPort(port_info) => {
                        println!("UspPort");
                        if port_info.vid == NORDIC_USB_VID && port_info.pid == NORDIC_USB_PID {
                            result.push(port.port_name);
                        }
                    },
                    SerialPortType::PciPort => { println!("PciPort")},
                    SerialPortType::BluetoothPort => { println!("BluetoothPort")},
                    SerialPortType::Unknown => { result.push(port.port_name)},
                }
            }
        }
        result
    }
}



impl Decoder for PacketCodec {
    type Item = Bytes;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> std::result::Result<Option<Self::Item>, Self::Error> {
        // We need at least two bytes for start/end packet.
        if src.len() < 2 {
            return Ok(None);
        }

        let frame_start = src.as_ref().iter().position(|b| *b == SLIP_END);

        if let Some(n) = frame_start {

            for frame_end in n+1..src.len() {
                if src[frame_end] == SLIP_END {
                    let data = src.split_to(frame_end + 1);
                    //src.advance(frame_end);
                    return match slip::decode(&data) {
                        Ok(packet) => {
                            Ok(Some(Bytes::from(packet)))
                        },
                        Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid SLIP packet")),
                    };
                }
            }
        }

        Ok(None)
    }
}

impl Encoder<&[u8]> for PacketCodec {
    type Error = io::Error;

    fn encode(&mut self, item: &[u8], dst: &mut BytesMut) -> std::result::Result<(), Self::Error> {
        dst.put(Bytes::from(slip::encode(item)));
        Ok(())
    }
}


mod tests {
    use crate::transport::serial;

    #[test]
    fn serial_test() {
        let mut port = serial::SerialTransport::new(serial::DEFAULT_TTY);
        while let Some(packet_data) = port.next_packet().await {
            let test = packet_data;
        }
    }
}