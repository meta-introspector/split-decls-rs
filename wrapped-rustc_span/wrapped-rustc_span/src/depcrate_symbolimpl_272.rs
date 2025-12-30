// Generated macro for impl_272 (impl)
macro_rules! Depcrate_symbolimpl_272 {
() => {
// Module: crate::symbol
// Provides: {"impl_272"}
// Dependencies: {}
impl < CTX > HashStable < CTX > for Symbol { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . as_str () . hash_stable (hcx , hasher) ; } }
};
}
