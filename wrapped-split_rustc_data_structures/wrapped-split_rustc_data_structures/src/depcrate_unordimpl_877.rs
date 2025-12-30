// Generated macro for impl_877 (impl)
macro_rules! Depcrate_unordimpl_877 {
() => {
// Module: crate::unord
// Provides: {"impl_877"}
// Dependencies: {}
impl < HCX , K : Hash + Eq + HashStable < HCX > , V : HashStable < HCX > > HashStable < HCX > for UnordMap < K , V > { # [inline] fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { hash_iter_order_independent (self . inner . iter () , hcx , hasher) ; } }
};
}
