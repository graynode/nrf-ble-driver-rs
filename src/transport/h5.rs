
use crate::{Error, Result};

const H5_HEADER_LENGTH: u8 = 4;

const SEQ_NUMBER_MASK: u8 = 0x07;
const ACK_NUMBER_MASK: u8 = 0x07;
const ACK_NUMBER_POS: u8 = 3;
const CRC_PRESENT_MASK: u8 = 0x01;
const CRC_PRESENT_POS: u8 = 6;
const RELIABLE_PACKET_MASK: u8 = 0x01;
const RELIABLE_PACKET_POS: u8 = 7;

const PACKET_TYPE_MASK: u8 = 0x0F;
const PAYLOAD_LENGTH_FIRST_NIBBLE_MASK: u16 = 0x000F;
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
    Unknown(u8),
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
            unknown => return H5PacketType::Unknown(unknown),
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
    Unknown(u8),
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
            unknown => ControlPacketType::Unknown(unknown),
        }
    }
}


pub fn decode_h5(slip_data: &[u8]) {
    let event = H5PacketType::from(slip_data[1] & PACKET_TYPE_MASK);
    let test = event as u8;
    println!("Event: {:?}, {:?}", event, test);

}