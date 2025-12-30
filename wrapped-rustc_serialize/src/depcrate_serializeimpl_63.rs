// Generated macro for impl_63 (impl)
macro_rules! Depcrate_serializeimpl_63 {
() => {
// Module: crate::serialize
// Provides: {"impl_63"}
// Dependencies: {}
impl < D : Decoder , T : Decodable < D > > Decodable < D > for Arc < T > { fn decode (d : & mut D) -> Arc < T > { Arc :: new (Decodable :: decode (d)) } }
};
}
