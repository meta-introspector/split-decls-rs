// Generated macro for u32_key (function)
macro_rules! Depcrate_writeru32_key {
() => {
// Module: crate::writer
// Provides: {"u32_key"}
// Dependencies: {}
# [doc = " Return the given u32 encoded in big-endian."] pub fn u32_key (cp : u32) -> [u8 ; 4] { cp . to_be_bytes () }
};
}
