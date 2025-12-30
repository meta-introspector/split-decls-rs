// Generated macro for Drain (struct)
macro_rules! Depcrate_collections_hash_setDrain {
() => {
// Module: crate::collections::hash::set
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator over the items of a `HashSet`."] # [doc = ""] # [doc = " This `struct` is created by the [`drain`] method on [`HashSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`drain`]: HashSet::drain"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashSet;"] # [doc = ""] # [doc = " let mut a = HashSet::from([1, 2, 3]);"] # [doc = ""] # [doc = " let mut drain = a.drain();"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "hashset_drain_ty")] pub struct Drain < 'a , K : 'a > { base : base :: Drain < 'a , K > , }
};
}
