// Generated macro for impl_116 (impl)
macro_rules! Depcrate_byte_phfimpl_116 {
() => {
// Module: crate::byte_phf
// Provides: {"impl_116"}
// Dependencies: {}
impl PerfectByteHashMap < [u8] > { # [doc = " Creates an instance from pre-existing bytes. See [`Self::as_bytes`]."] # [inline] pub fn from_bytes (bytes : & [u8]) -> & Self { unsafe { core :: mem :: transmute (bytes) } } }
};
}
