// Generated macro for impl_87 (impl)
macro_rules! Depcrate_serializeimpl_87 {
() => {
// Module: crate::serialize
// Provides: {"impl_87"}
// Dependencies: {}
impl < D : Decoder , T : Decodable < D > > Decodable < D > for Arc < [T] > { fn decode (d : & mut D) -> Arc < [T] > { let vec : Vec < T > = Decodable :: decode (d) ; vec . into () } }
};
}
