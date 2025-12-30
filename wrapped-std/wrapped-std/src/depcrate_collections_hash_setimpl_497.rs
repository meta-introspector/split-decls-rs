// Generated macro for impl_497 (impl)
macro_rules! Depcrate_collections_hash_setimpl_497 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_497"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T , S > Iterator for Difference < 'a , T , S > where T : Eq + Hash , S : BuildHasher , { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < & 'a T > { loop { let elt = self . iter . next () ? ; if ! self . other . contains (elt) { return Some (elt) ; } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } # [inline] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . iter . fold (init , | acc , elt | if self . other . contains (elt) { acc } else { f (acc , elt) }) } }
};
}
