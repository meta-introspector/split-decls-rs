// Generated macro for impl_883 (impl)
macro_rules! Depcrate_unordimpl_883 {
() => {
// Module: crate::unord
// Provides: {"impl_883"}
// Dependencies: {}
impl < HCX , V : Hash + Eq + HashStable < HCX > > HashStable < HCX > for UnordBag < V > { # [inline] fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { hash_iter_order_independent (self . inner . iter () , hcx , hasher) ; } }
};
}
