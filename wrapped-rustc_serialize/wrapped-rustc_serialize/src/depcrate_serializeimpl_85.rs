// Generated macro for impl_85 (impl)
macro_rules! Depcrate_serializeimpl_85 {
() => {
// Module: crate::serialize
// Provides: {"impl_85"}
// Dependencies: {}
impl < D : Decoder , T : Decodable < D > > Decodable < D > for Rc < [T] > { fn decode (d : & mut D) -> Rc < [T] > { let vec : Vec < T > = Decodable :: decode (d) ; vec . into () } }
};
}
