use byteorder::{BigEndian, ByteOrder};

pub struct Page {
    data: Vec<u8>,
}

impl Page {
    pub fn new(block_size: usize) -> Self {
        Self {
            data: vec![0; block_size],
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { data: bytes }
    }

    pub fn get_int(&self, offset: usize) -> i32 {
        let bytes = &self.data[offset..offset + 4];
        BigEndian::read_i32(bytes)
    }

    pub fn set_int(&mut self, offset: usize, val: i32) {
        let bytes = &mut self.data[offset..offset + 4];
        BigEndian::write_i32(bytes, val);
    }

    pub fn get_bytes(&self, offset: usize) -> Vec<u8> {
        let len = self.get_int(offset) as usize;
        let start = offset + 4;
        let end = start + len;
        self.data[start..end].to_vec()
    }

    pub fn set_bytes(&mut self, offset: usize, val: &[u8]) {
        let len = val.len() as i32;
        self.set_int(offset, len);
        let start = offset + 4;
        let end = start + val.len();
        self.data[start..end].copy_from_slice(val);
    }

    pub fn get_string(&self, offset: usize) -> String {
        let bytes = self.get_bytes(offset);
        String::from_utf8(bytes).expect("Invalid ASCII data")
    }

    pub fn set_string(&mut self, offset: usize, val: &str) {
        let bytes = val.as_bytes();
        self.set_bytes(offset, bytes);
    }

    pub fn max_length(strlen: usize) -> usize {
        // 4 bytes for length prefix (i32), 1 byte per ASCII char
        4 + strlen
    }

    pub fn contents(&self) -> &[u8] {
        &self.data
    }

    pub fn contents_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}
