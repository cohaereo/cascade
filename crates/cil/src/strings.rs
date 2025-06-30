use crate::meta::StringIndex;

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
