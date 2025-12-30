// Generated macro for crc32 (function)
macro_rules! Depcrate_crc32crc32 {
() => {
// Module: crate::crc32
// Provides: {"crc32"}
// Dependencies: {}
pub fn crc32 (start : u32 , buf : & [u8]) -> u32 { if buf . len () < 64 { return crc32_braid (start , buf) ; } let mut crc_state = Crc32Fold :: new_with_initial (start) ; crc_state . fold (buf , start) ; crc_state . finish () }
};
}
