// Generated macro for impl_848 (impl)
macro_rules! Depcrate_crypto_ring_hashimpl_848 {
() => {
// Module: crate::crypto::ring::hash
// Provides: {"impl_848"}
// Dependencies: {}
impl crypto :: hash :: Hash for Hash { fn start (& self) -> Box < dyn crypto :: hash :: Context > { Box :: new (Context (digest :: Context :: new (self . 0))) } fn hash (& self , bytes : & [u8]) -> crypto :: hash :: Output { let mut ctx = digest :: Context :: new (self . 0) ; ctx . update (bytes) ; convert (ctx . finish ()) } fn output_len (& self) -> usize { self . 0 . output_len () } fn algorithm (& self) -> HashAlgorithm { self . 1 } fn fips (& self) -> bool { super :: fips () } }
};
}
