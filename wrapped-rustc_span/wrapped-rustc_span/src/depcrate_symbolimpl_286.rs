// Generated macro for impl_286 (impl)
macro_rules! Depcrate_symbolimpl_286 {
() => {
// Module: crate::symbol
// Provides: {"impl_286"}
// Dependencies: {}
impl < CTX > HashStable < CTX > for ByteSymbol { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . as_byte_str () . hash_stable (hcx , hasher) ; } }
};
}
