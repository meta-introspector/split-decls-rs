// Generated macro for impl_49 (impl)
macro_rules! Depcrate_serializeimpl_49 {
() => {
// Module: crate::serialize
// Provides: {"impl_49"}
// Dependencies: {}
impl < D : Decoder , T : Decodable < D > > Decodable < D > for Option < T > { fn decode (d : & mut D) -> Option < T > { match d . read_u8 () { 0 => None , 1 => Some (Decodable :: decode (d)) , _ => panic ! ("Encountered invalid discriminant while decoding `Option`.") , } } }
};
}
