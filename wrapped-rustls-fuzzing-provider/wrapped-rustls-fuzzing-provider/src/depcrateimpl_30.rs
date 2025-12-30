// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl hash :: Hash for Hash { fn start (& self) -> Box < dyn hash :: Context > { Box :: new (HashContext) } fn hash (& self , _data : & [u8]) -> hash :: Output { hash :: Output :: new (HASH_OUTPUT) } fn algorithm (& self) -> HashAlgorithm { HashAlgorithm :: from (0xff) } fn output_len (& self) -> usize { 32 } }
};
}
