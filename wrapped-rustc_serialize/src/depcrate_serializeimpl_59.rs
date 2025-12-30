// Generated macro for impl_59 (impl)
macro_rules! Depcrate_serializeimpl_59 {
() => {
// Module: crate::serialize
// Provides: {"impl_59"}
// Dependencies: {}
impl < D : Decoder , T : Decodable < D > + Copy > Decodable < D > for Cell < T > { fn decode (d : & mut D) -> Cell < T > { Cell :: new (Decodable :: decode (d)) } }
};
}
