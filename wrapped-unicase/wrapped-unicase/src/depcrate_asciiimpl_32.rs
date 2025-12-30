// Generated macro for impl_32 (impl)
macro_rules! Depcrate_asciiimpl_32 {
() => {
// Module: crate::ascii
// Provides: {"impl_32"}
// Dependencies: {}
impl < S : AsRef < str > > Hash for Ascii < S > { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { for byte in self . as_ref () . bytes () . map (| b | b . to_ascii_lowercase ()) { hasher . write_u8 (byte) ; } hasher . write_u8 (0xFF) ; } }
};
}
