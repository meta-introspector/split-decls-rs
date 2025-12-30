// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl From < XXH128_hash_t > for u128 { fn from (value : XXH128_hash_t) -> Self { u128 :: from (value . high64) << 64 | u128 :: from (value . low64) } }
};
}
