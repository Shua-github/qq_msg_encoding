pub struct SimpleRng {
    seed: u32,
}

impl SimpleRng {    #[allow(dead_code)]
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        self.seed
    }

    #[allow(dead_code)]
    pub fn gen_uuid_v4(&mut self) -> String {
        let mut parts = [0u32; 4];
        for i in 0..4 {
            parts[i] = self.next_u32();
        }

        let bytes = parts.iter().flat_map(|n| n.to_be_bytes()).collect::<Vec<_>>();

        let mut uuid_bytes = bytes;
        uuid_bytes[6] = (uuid_bytes[6] & 0x0f) | 0x40;
        uuid_bytes[8] = (uuid_bytes[8] & 0x3f) | 0x80;

        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            u32::from_be_bytes([uuid_bytes[0], uuid_bytes[1], uuid_bytes[2], uuid_bytes[3]]),
            u16::from_be_bytes([uuid_bytes[4], uuid_bytes[5]]),
            u16::from_be_bytes([uuid_bytes[6], uuid_bytes[7]]),
            u16::from_be_bytes([uuid_bytes[8], uuid_bytes[9]]),
            u64::from_be_bytes([
                uuid_bytes[10], uuid_bytes[11], uuid_bytes[12], uuid_bytes[13],
                uuid_bytes[14], uuid_bytes[15], 0, 0
            ]) >> 16
        )
    }
}