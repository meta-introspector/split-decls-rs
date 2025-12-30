// Generated macro for impl_38 (impl)
macro_rules! Depcrate_serializeimpl_38 {
() => {
// Module: crate::serialize
// Provides: {"impl_38"}
// Dependencies: {}
impl < D : Decoder , T : Decodable < D > > Decodable < D > for Rc < T > { fn decode (d : & mut D) -> Rc < T > { Rc :: new (Decodable :: decode (d)) } }
};
}
