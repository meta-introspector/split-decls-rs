// Generated macro for impl_96 (impl)
macro_rules! Depcrate_sinkimpl_96 {
() => {
// Module: crate::sink
// Provides: {"impl_96"}
// Dependencies: {}
impl SinkFinish { # [doc = " Return the total number of bytes searched."] # [inline] pub fn byte_count (& self) -> u64 { self . byte_count } # [doc = " If binary detection is enabled and if binary data was found, then this"] # [doc = " returns the absolute byte offset of the first detected byte of binary"] # [doc = " data."] # [doc = ""] # [doc = " Note that since this is an absolute byte offset, it cannot be relied"] # [doc = " upon to index into any addressable memory."] # [inline] pub fn binary_byte_offset (& self) -> Option < u64 > { self . binary_byte_offset } }
};
}
