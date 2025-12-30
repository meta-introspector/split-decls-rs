// Generated macro for impl_117 (impl)
macro_rules! Depcrate_byte_phfimpl_117 {
() => {
// Module: crate::byte_phf
// Provides: {"impl_117"}
// Dependencies: {}
impl < Store > PerfectByteHashMap < Store > where Store : AsRef < [u8] > + ? Sized , { # [doc = " Converts from `PerfectByteHashMap<AsRef<[u8]>>` to `&PerfectByteHashMap<[u8]>`"] # [inline] pub fn as_borrowed (& self) -> & PerfectByteHashMap < [u8] > { PerfectByteHashMap :: from_bytes (self . 0 . as_ref ()) } }
};
}
