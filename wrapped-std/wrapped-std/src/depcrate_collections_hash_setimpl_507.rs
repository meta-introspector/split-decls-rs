// Generated macro for impl_507 (impl)
macro_rules! Depcrate_collections_hash_setimpl_507 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_507"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T , S > Iterator for Union < 'a , T , S > where T : Eq + Hash , S : BuildHasher , { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < & 'a T > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } # [inline] fn count (self) -> usize { self . iter . count () } # [inline] fn fold < B , F > (self , init : B , f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . iter . fold (init , f) } }
};
}
