pub struct PageId(pub u64);

impl PageId {
    pub fn to_u64(&self) -> u64 {
        self.0 as u64
    }
}
