// Generated macro for impl_705 (impl)
macro_rules! Depcrate_stable_hasherimpl_705 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_705"}
// Dependencies: {}
impl < K , V , HCX > HashStable < HCX > for :: std :: collections :: BTreeMap < K , V > where K : HashStable < HCX > + StableOrd , V : HashStable < HCX > , { fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { self . len () . hash_stable (hcx , hasher) ; for entry in self . iter () { entry . hash_stable (hcx , hasher) ; } } }
};
}
