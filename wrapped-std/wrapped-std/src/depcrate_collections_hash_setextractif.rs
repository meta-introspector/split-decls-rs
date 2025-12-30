// Generated macro for ExtractIf (struct)
macro_rules! Depcrate_collections_hash_setExtractIf {
() => {
// Module: crate::collections::hash::set
// Provides: {"ExtractIf"}
// Dependencies: {}
# [doc = " A draining, filtering iterator over the items of a `HashSet`."] # [doc = ""] # [doc = " This `struct` is created by the [`extract_if`] method on [`HashSet`]."] # [doc = ""] # [doc = " [`extract_if`]: HashSet::extract_if"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashSet;"] # [doc = ""] # [doc = " let mut a = HashSet::from([1, 2, 3]);"] # [doc = ""] # [doc = " let mut extract_ifed = a.extract_if(|v| v % 2 == 0);"] # [doc = " ```"] # [stable (feature = "hash_extract_if" , since = "1.88.0")] pub struct ExtractIf < 'a , K , F > { base : base :: ExtractIf < 'a , K , F > , }
};
}
