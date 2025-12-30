// Generated macro for impl_36 (impl)
macro_rules! Depcrate_serializeimpl_36 {
() => {
// Module: crate::serialize
// Provides: {"impl_36"}
// Dependencies: {}
impl < D : Decoder , T : Decodable < D > > Decodable < D > for Box < [T] > { fn decode (d : & mut D) -> Box < [T] > { let v : Vec < T > = Decodable :: decode (d) ; v . into_boxed_slice () } }
};
}
