// Generated macro for impl_430 (impl)
macro_rules! Depcrateimpl_430 {
() => {
// Module: crate
// Provides: {"impl_430"}
// Dependencies: {}
impl < D : Decoder > Decodable < D > for RelativeBytePos { fn decode (d : & mut D) -> RelativeBytePos { RelativeBytePos (d . read_u32 ()) } }
};
}
