// Generated macro for impl_13 (impl)
macro_rules! Depcrate_bytearrayimpl_13 {
() => {
// Module: crate::bytearray
// Provides: {"impl_13"}
// Dependencies: {}
impl < const N : usize > ByteArray < N > { # [doc = " Wrap an existing [array] into a `ByteArray`."] pub const fn new (bytes : [u8 ; N]) -> Self { ByteArray { bytes } } # [doc = " Unwrap the byte array underlying this `ByteArray`."] pub const fn into_array (self) -> [u8 ; N] { self . bytes } fn from_ref (bytes : & [u8 ; N]) -> & Self { unsafe { & * (bytes as * const [u8 ; N] as * const ByteArray < N >) } } }
};
}
