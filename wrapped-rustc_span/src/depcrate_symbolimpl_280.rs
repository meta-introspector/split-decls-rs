// Generated macro for impl_280 (impl)
macro_rules! Depcrate_symbolimpl_280 {
() => {
// Module: crate::symbol
// Provides: {"impl_280"}
// Dependencies: {}
impl < CTX > HashStable < CTX > for Symbol { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . as_str () . hash_stable (hcx , hasher) ; } }
};
}
