// Generated macro for impl_1451 (impl)
macro_rules! Depcrate_crypto_hashimpl_1451 {
() => {
// Module: crate::crypto::hash
// Provides: {"impl_1451"}
// Dependencies: {}
# [cfg (all (test , any (feature = "aws-lc-rs" , feature = "ring")))] impl Hash for FakeHash { fn algorithm (& self) -> HashAlgorithm { todo ! () } fn fips (& self) -> bool { false } fn hash (& self , _bytes : & [u8]) -> Output { todo ! () } fn output_len (& self) -> usize { todo ! () } fn start (& self) -> Box < dyn Context > { Box :: new (FakeHashContext) } }
};
}
