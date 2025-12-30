// Generated macro for impl_414 (impl)
macro_rules! Depcrateimpl_414 {
() => {
// Module: crate
// Provides: {"impl_414"}
// Dependencies: {}
impl < D : Decoder > Decodable < D > for RelativeBytePos { fn decode (d : & mut D) -> RelativeBytePos { RelativeBytePos (d . read_u32 ()) } }
};
}
