
use crate::{Error, Result, transport::slip};
use std::vec::{Vec};

const H5_HEADER_LENGTH: usize = 4;

const SEQ_NUMBER_MASK: u8 = 0x07;
const ACK_NUMBER_MASK: u8 = 0x07;
const ACK_NUMBER_POS: u8 = 3;
const CRC_PRESENT_MASK: u8 = 0x01;
const CRC_PRESENT_POS: u8 = 6;
const RELIABLE_PACKET_MASK: u8 = 0x01;
const RELIABLE_PACKET_POS: u8 = 7;

const PACKET_TYPE_MASK: u8 = 0x0F;
const PAYLOAD_LENGTH_FIRST_NIBBLE_MASK: u8 = 0x0F;
const PAYLOAD_LENGTH_SECOND_NIBBLE_MASK: u16 = 0x0FF0;
const PAYLOAD_LENGTH_OFFSET: u8 = 4;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(u8)]
enum H5PacketType {
    Ack = 0,
    HciCommand = 1,
    AclData = 2,
    SyncData = 3,
    HciEvent = 4,
    Reset = 5,
    VendorSpecific = 14,
    LinkControl = 15,
    Unknown,
}

impl From<u8> for H5PacketType {
    fn from(orig: u8) -> Self {
        match orig {
            0 => return H5PacketType::Ack,
            1 => return H5PacketType::HciCommand,
            2 => return H5PacketType::AclData,
            3 => return H5PacketType::SyncData,
            4 => return H5PacketType::HciEvent,
            5 => return H5PacketType::Reset,
            14 => return H5PacketType::VendorSpecific,
            15 => return H5PacketType::LinkControl,
            _ => return H5PacketType::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(u8)]
enum ControlPacketType {
    Reset = 0,
    Ack = 1,
    Sync = 2,
    SyncResponse = 3,
    SyncConfig = 4,
    SyncConfigResponse = 5,
    Last = 10,
    Unknown,
}

impl From<u8> for ControlPacketType {
    fn from(orig: u8) -> Self {
        match orig {
            0 => ControlPacketType::Reset,
            1 => ControlPacketType::Ack,
            2 => ControlPacketType::Sync,
            3 => ControlPacketType::SyncResponse,
            4 => ControlPacketType::SyncConfig,
            5 => ControlPacketType::SyncConfigResponse,
            10 => ControlPacketType::Last,
            _ => ControlPacketType::Unknown,
        }
    }
}

enum H5State {
    Start,
    Reset,
    Uninitialized,
    Initialized,
    Active,
    Failed,
    Closed,
    NoResponse,
    Unknown,
}


#[derive(Debug, Clone)]
struct H5Packet {
    pub seq_number: u8,
    pub ack_number: u8,
    pub crc_present: bool,
    pub reliable_packet: bool,
    pub packet_type: H5PacketType,
    pub payload_length: usize,
    pub header_checksum: u8,
    pub payload: Vec<u8>,
}

impl H5Packet {

    pub fn new(data: &[u8]) -> Result<H5Packet> {
        if data.len() < H5_HEADER_LENGTH {
            return Err(Error::H5Error);
        }

        let payload_length = usize::from(((data[1] >> PAYLOAD_LENGTH_OFFSET) & PAYLOAD_LENGTH_FIRST_NIBBLE_MASK) +
                                             (data[2] << PAYLOAD_LENGTH_OFFSET));
        let crc_present = (data[0] >> CRC_PRESENT_POS) == CRC_PRESENT_MASK;
        let calculated_payload_size: usize = payload_length as usize + H5_HEADER_LENGTH + if crc_present {2} else {0};
        let header_checksum = data[3];

        // Verify the packet length
        if data.len() != calculated_payload_size {
            return Err(Error::H5Error);
        }

        // Verify header checksum
        let calculated_header_checksum = H5Packet::calc_header_checksum(data);
        if calculated_header_checksum != header_checksum {
            return Err(Error::H5Error);
        }

        if crc_present {
            // calc checksum here and error out 
        }

        Ok(H5Packet { seq_number: data[0] & SEQ_NUMBER_MASK,
                      ack_number: (data[0] >> ACK_NUMBER_POS) & ACK_NUMBER_MASK,
                      crc_present,
                      reliable_packet: (data[0] >> RELIABLE_PACKET_POS) == RELIABLE_PACKET_MASK,
                      packet_type: H5PacketType::from(data[1] & PACKET_TYPE_MASK),
                      payload_length,
                      header_checksum,
                      payload: if payload_length > 0 {Vec::from_iter(data[H5_HEADER_LENGTH..H5_HEADER_LENGTH + payload_length].iter().cloned())} else {Vec::new()} })
    }

    fn calc_header_checksum(header: &[u8]) -> u8 {

        let mut checksum: u16 = header[0].into();
        checksum += header[1] as u16;
        checksum += header[2] as u16;
        checksum &= 0xFF;
        checksum = !checksum + 1;
        return checksum as u8;
    }
}

pub fn decode_h5(slip_data: &[u8]) {
    let h5packet = H5Packet::new(slip_data).unwrap();
    println!("Event: {:?}", h5packet);

}