pub struct ADNLPacket<const PAYLOAD_SIZE: usize> {
    pub size: [u8; 4],
    pub nonce: [u8; 32],
    pub payload: [u8; PAYLOAD_SIZE],
    pub sha256: [u8; 32],
}
