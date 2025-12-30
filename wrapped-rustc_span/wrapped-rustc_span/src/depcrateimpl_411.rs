// Generated macro for impl_411 (impl)
macro_rules! Depcrateimpl_411 {
() => {
// Module: crate
// Provides: {"impl_411"}
// Dependencies: {}
impl < D : Decoder > Decodable < D > for BytePos { fn decode (d : & mut D) -> BytePos { BytePos (d . read_u32 ()) } }
};
}
