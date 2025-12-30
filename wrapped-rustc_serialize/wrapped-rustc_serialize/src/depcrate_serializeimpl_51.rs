// Generated macro for impl_51 (impl)
macro_rules! Depcrate_serializeimpl_51 {
() => {
// Module: crate::serialize
// Provides: {"impl_51"}
// Dependencies: {}
impl < D : Decoder , T1 : Decodable < D > , T2 : Decodable < D > > Decodable < D > for Result < T1 , T2 > { fn decode (d : & mut D) -> Result < T1 , T2 > { match d . read_u8 () { 0 => Ok (T1 :: decode (d)) , 1 => Err (T2 :: decode (d)) , _ => panic ! ("Encountered invalid discriminant while decoding `Result`.") , } } }
};
}
