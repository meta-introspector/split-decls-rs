// Generated macro for impl_209 (impl)
macro_rules! Depcrate_stream_bstrimpl_209 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_209"}
// Dependencies: {}
impl BStr { # [doc = " Make a stream out of a byte slice-like."] # [inline] pub fn new < B : ? Sized + AsRef < [u8] > > (bytes : & B) -> & Self { Self :: from_bytes (bytes . as_ref ()) } # [inline] fn from_bytes (slice : & [u8]) -> & Self { unsafe { core :: mem :: transmute (slice) } } # [inline] fn as_bytes (& self) -> & [u8] { & self . 0 } }
};
}
