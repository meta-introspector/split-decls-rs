// Generated macro for impl_65 (impl)
macro_rules! Depcrate_serializeimpl_65 {
() => {
// Module: crate::serialize
// Provides: {"impl_65"}
// Dependencies: {}
impl < D : Decoder , T : Decodable < D > > Decodable < D > for Box < T > { fn decode (d : & mut D) -> Box < T > { Box :: new (Decodable :: decode (d)) } }
};
}
