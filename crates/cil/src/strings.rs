use crate::meta::{StringIndex, Token, TokenKind};

pub struct StringHeap {
    data: Vec<u8>,
}

impl StringHeap {
    pub fn new(data: Vec<u8>) -> Self {
        StringHeap { data }
    }

    pub fn get(&self, index: StringIndex) -> Option<String> {
        let mut bytes = Vec::new();
        for i in index.0 as usize..self.data.len() {
            let byte = self.data[i];
            if byte == 0 {
                break;
            }
            bytes.push(byte);
        }

        Some(String::from_utf8(bytes).ok()?)
    }

    pub fn try_get(&self, index: StringIndex) -> Result<String, std::string::FromUtf8Error> {
        let mut bytes = Vec::new();
        for i in index.0 as usize..self.data.len() {
            let byte = self.data[i];
            if byte == 0 {
                break;
            }
            bytes.push(byte);
        }

        String::from_utf8(bytes)
    }
}

pub struct UserStringHeap {
    data: Vec<u8>,
}

impl UserStringHeap {
    pub fn new(data: Vec<u8>) -> Self {
        UserStringHeap { data }
    }

    pub fn get(&self, token: Token) -> Option<String> {
        assert_eq!(
            token.kind(),
            TokenKind::UserString,
            "Token is not a UserString"
        );

        let mut offset = token.index() as usize;
        let mut len = 0;
        // Varint encoding
        // For unsigned integers:
        //   - If the value lies between 0 (0x00) and 127 (0x7F), inclusive, encode as a one-byte integer (bit 7 is clear, value held in bits 6 through 0)
        //   - If the value lies between 28 (0x80) and 214-1 (0x3FFF), inclusive, encode as a 2-byte integer with bit 15 set, bit 14 clear (value held in bits 13 through 0)
        //   - Otherwise, encode as a 4-byte integer, with bit 31 set, bit 30 set, bit 29 clear (value held in bits 28 through 0)
        let first_byte = self.data[offset];
        if first_byte & 0x80 == 0 {
            // One-byte integer (0-127)
            len = first_byte as usize;
            offset += 1;
        } else if first_byte & 0xC0 == 0x80 {
            // Two-byte integer (128-16383)
            let second_byte = self.data[offset + 1];
            len = (((first_byte & 0x3F) as usize) << 8) | (second_byte as usize);
            offset += 2;
        } else if first_byte & 0xE0 == 0xC0 {
            // Four-byte integer
            let second_byte = self.data[offset + 1];
            let third_byte = self.data[offset + 2];
            let fourth_byte = self.data[offset + 3];
            len = (((first_byte & 0x1F) as usize) << 24)
                | ((second_byte as usize) << 16)
                | ((third_byte as usize) << 8)
                | (fourth_byte as usize);
            offset += 4;
        }

        let mut values: Vec<u16> = Vec::new();
        let end = offset + len;
        while (offset + 1) < end && offset + 1 < self.data.len() {
            let value = u16::from_le_bytes([self.data[offset], self.data[offset + 1]]);
            values.push(value);
            offset += 2;
        }

        Some(String::from_utf16(&values).ok()?)
    }
}
