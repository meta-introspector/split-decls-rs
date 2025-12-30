// Generated macro for impl_272 (impl)
macro_rules! Depcrate_stream_bytesimpl_272 {
() => {
// Module: crate::stream::bytes
// Provides: {"impl_272"}
// Dependencies: {}
impl Bytes { # [doc = " Make a stream out of a byte slice-like."] # [inline] pub fn new < B : ? Sized + AsRef < [u8] > > (bytes : & B) -> & Self { Self :: from_bytes (bytes . as_ref ()) } # [inline] fn from_bytes (slice : & [u8]) -> & Self { unsafe { core :: mem :: transmute (slice) } } # [inline] fn as_bytes (& self) -> & [u8] { & self . 0 } }
};
}
