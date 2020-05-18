pub struct Data<'b> {
    bytes: &'b [u8],
    start: usize,
}

impl Data<'_> {
    pub fn new(bytes: &[u8]) -> Data {
        Data { bytes, start: 0 }
    }

    pub fn read_bytes(&mut self, size: usize) -> Option<&[u8]> {
        if self.start + size <= self.bytes.len() {
            let start = self.start;
            self.start += size;

            Some(&self.bytes[start..start + size])
        } else {
            None
        }
    }

    pub fn skip(&mut self, size: usize) {
        self.start += size;
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        let bytes = self.read_bytes(2)?;

        let mut buf = [0; 2];
        buf.copy_from_slice(&bytes[..2]);

        Some(u16::from_le_bytes(buf))
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        let bytes = self.read_bytes(1)?;
        Some(bytes[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_bytes() {
        let mut data = Data::new(&[0x0, 0x1, 0x2]);

        assert_eq!(data.read_bytes(1).unwrap(), &[0x0]);
        assert_eq!(data.read_bytes(2).unwrap(), &[0x1, 0x2]);

        assert!(data.read_bytes(1).is_none());
    }

    #[test]
    fn skip() {
        let mut data = Data::new(&[0x0, 0x1, 0x2]);

        data.skip(2);
        data.skip(1);

        assert!(data.read_bytes(1).is_none());
    }

    #[test]
    fn read_u16() {
        let mut data = Data::new(&[0xcd, 0xab]);
        assert_eq!(data.read_u16(), Some(0xabcd));
        assert_eq!(data.read_u16(), None);
    }

    #[test]
    fn read_u8() {
        let mut data = Data::new(&[0xab]);
        assert_eq!(data.read_u8(), Some(0xab));
        assert_eq!(data.read_u8(), None);
    }
}
