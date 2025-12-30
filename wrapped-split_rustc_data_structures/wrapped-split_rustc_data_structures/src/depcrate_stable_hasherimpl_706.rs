// Generated macro for impl_706 (impl)
macro_rules! Depcrate_stable_hasherimpl_706 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_706"}
// Dependencies: {}
impl < K , HCX > HashStable < HCX > for :: std :: collections :: BTreeSet < K > where K : HashStable < HCX > + StableOrd , { fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { self . len () . hash_stable (hcx , hasher) ; for entry in self . iter () { entry . hash_stable (hcx , hasher) ; } } }
};
}
