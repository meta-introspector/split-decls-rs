// Generated macro for encode_offset (function)
macro_rules! Depcrate_encoding_blocks_compressedencode_offset {
() => {
// Module: crate::encoding::blocks::compressed
// Provides: {"encode_offset"}
// Dependencies: {}
fn encode_offset (len : u32) -> (u8 , u32 , usize) { let log = len . ilog2 () ; let lower = len & ((1 << log) - 1) ; (log as u8 , lower , log as usize) }
};
}
