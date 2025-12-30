// Generated macro for impl_475 (impl)
macro_rules! Depcrate_collections_hash_setimpl_475 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_475"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , S > IntoIterator for HashSet < T , S > { type Item = T ; type IntoIter = IntoIter < T > ; # [doc = " Creates a consuming iterator, that is, one that moves each value out"] # [doc = " of the set in arbitrary order. The set cannot be used after calling"] # [doc = " this."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashSet;"] # [doc = " let mut set = HashSet::new();"] # [doc = " set.insert(\"a\".to_string());"] # [doc = " set.insert(\"b\".to_string());"] # [doc = ""] # [doc = " // Not possible to collect to a Vec<String> with a regular `.iter()`."] # [doc = " let v: Vec<String> = set.into_iter().collect();"] # [doc = ""] # [doc = " // Will print in an arbitrary order."] # [doc = " for x in &v {"] # [doc = "     println!(\"{x}\");"] # [doc = " }"] # [doc = " ```"] # [inline] # [rustc_lint_query_instability] fn into_iter (self) -> IntoIter < T > { IntoIter { base : self . base . into_iter () } } }
};
}
