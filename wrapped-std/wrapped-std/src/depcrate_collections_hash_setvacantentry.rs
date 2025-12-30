// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_collections_hash_setVacantEntry {
() => {
// Module: crate::collections::hash::set
// Provides: {"VacantEntry"}
// Dependencies: {}
# [doc = " A view into a vacant entry in a `HashSet`."] # [doc = " It is part of the [`Entry`] enum."] # [doc = ""] # [doc = " [`Entry`]: enum.Entry.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(hash_set_entry)]"] # [doc = ""] # [doc = " use std::collections::hash_set::{Entry, HashSet};"] # [doc = ""] # [doc = " let mut set = HashSet::<&str>::new();"] # [doc = ""] # [doc = " let entry_v = match set.entry(\"a\") {"] # [doc = "     Entry::Vacant(view) => view,"] # [doc = "     Entry::Occupied(_) => unreachable!(),"] # [doc = " };"] # [doc = " entry_v.insert();"] # [doc = " assert!(set.contains(\"a\") && set.len() == 1);"] # [doc = ""] # [doc = " // Nonexistent key (insert)"] # [doc = " match set.entry(\"b\") {"] # [doc = "     Entry::Vacant(view) => view.insert(),"] # [doc = "     Entry::Occupied(_) => unreachable!(),"] # [doc = " }"] # [doc = " assert!(set.contains(\"b\") && set.len() == 2);"] # [doc = " ```"] # [unstable (feature = "hash_set_entry" , issue = "60896")] pub struct VacantEntry < 'a , T , S > { base : base :: VacantEntry < 'a , T , S > , }
};
}
