use crate::{Error, Result};

pub const SLIP_END: u8 = 0xC0;
const SLIP_ESC: u8 = 0xDB;
const SLIP_ESC_END: u8 = 0xDC;
const SLIP_ESC_ESC: u8 = 0xDD;


pub fn encode(input: &[u8]) -> Vec<u8> {
    let mut encoded_data = vec![SLIP_END];

    for byte in input {
        match *byte {
            SLIP_END => {
                encoded_data.extend([SLIP_ESC, SLIP_ESC_END]);
            }
            SLIP_ESC => {
                encoded_data.extend([SLIP_ESC, SLIP_ESC_ESC]);
            }
            _ => {
                encoded_data.push(*byte);
            }
        }
    }

    encoded_data.push(SLIP_END);
    
    encoded_data
}

pub fn decode(input: &[u8]) -> Result<Vec<u8>> {
    let mut decoded_data = vec![];
    let mut index = 0;

    while index < input.len() {

        match input[index] {
            SLIP_END => {
                
            }
            SLIP_ESC => {
                index += 1;
                if index == input.len() {
                    return Err(Error::SlipDecodingError);
                }
                
                match input[index] {
                    SLIP_ESC_END => {
                        decoded_data.push(SLIP_END);
                    }
                    SLIP_ESC_ESC => {
                        decoded_data.push(SLIP_ESC);
                    }
                    _ => {
                        return Err(Error::SlipDecodingError);
                    }
                }
            }
            _ => {
                decoded_data.push(input[index]);
            }
        }

        index += 1;
    }

    Ok(decoded_data)
}

#[cfg(test)]
mod tests {
    use crate::transport::slip;

    #[test]
    fn slip_encode_test() {
        let input: Vec<u8> = vec![0x01, 0xDB, 0x49, 0xC0, 0x15];
        let expected: Vec<u8> = vec![0xC0, 0x01, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15, 0xC0];

        let encoded = slip::encode(&input);
        assert_eq!(encoded, expected);
    }

    #[test]
    fn slip_decode_escapes_test() {
        let input: Vec<u8> = vec![0x01, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15, 0xC0];
        let expected: Vec<u8> = vec![0x01, 0xDB, 0x49, 0xC0, 0x15];

        let result: Vec<u8> = slip::decode(&input).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn slip_decode_no_escapes_test() {
        let input = vec![0x01, 0x49, 0x15, 0x02];
        let decoded = slip::decode(&input).unwrap();

        assert_eq!(input, decoded);
    }
}