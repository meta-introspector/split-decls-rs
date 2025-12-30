// Generated macro for impl_28 (impl)
macro_rules! Depcrate_serializeimpl_28 {
() => {
// Module: crate::serialize
// Provides: {"impl_28"}
// Dependencies: {}
impl < D : Decoder > Decodable < D > for NonZero < u32 > { fn decode (d : & mut D) -> Self { NonZero :: new (d . read_u32 ()) . unwrap () } }
};
}
