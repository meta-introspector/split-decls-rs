// Generated macro for impl_427 (impl)
macro_rules! Depcrateimpl_427 {
() => {
// Module: crate
// Provides: {"impl_427"}
// Dependencies: {}
impl < D : Decoder > Decodable < D > for BytePos { fn decode (d : & mut D) -> BytePos { BytePos (d . read_u32 ()) } }
};
}
