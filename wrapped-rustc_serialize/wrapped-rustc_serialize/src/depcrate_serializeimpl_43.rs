// Generated macro for impl_43 (impl)
macro_rules! Depcrate_serializeimpl_43 {
() => {
// Module: crate::serialize
// Provides: {"impl_43"}
// Dependencies: {}
impl < D : Decoder , const N : usize > Decodable < D > for [u8 ; N] { fn decode (d : & mut D) -> [u8 ; N] { let len = d . read_usize () ; assert ! (len == N) ; let mut v = [0u8 ; N] ; for i in 0 .. len { v [i] = Decodable :: decode (d) ; } v } }
};
}
