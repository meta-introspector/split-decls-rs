// Generated macro for OccupiedEntry (struct)
macro_rules! Depcrate_collections_hash_setOccupiedEntry {
() => {
// Module: crate::collections::hash::set
// Provides: {"OccupiedEntry"}
// Dependencies: {}
# [doc = " A view into an occupied entry in a `HashSet`."] # [doc = " It is part of the [`Entry`] enum."] # [doc = ""] # [doc = " [`Entry`]: enum.Entry.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(hash_set_entry)]"] # [doc = ""] # [doc = " use std::collections::hash_set::{Entry, HashSet};"] # [doc = ""] # [doc = " let mut set = HashSet::new();"] # [doc = " set.extend([\"a\", \"b\", \"c\"]);"] # [doc = ""] # [doc = " let _entry_o = set.entry(\"a\").insert();"] # [doc = " assert_eq!(set.len(), 3);"] # [doc = ""] # [doc = " // Existing key"] # [doc = " match set.entry(\"a\") {"] # [doc = "     Entry::Vacant(_) => unreachable!(),"] # [doc = "     Entry::Occupied(view) => {"] # [doc = "         assert_eq!(view.get(), &\"a\");"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(set.len(), 3);"] # [doc = ""] # [doc = " // Existing key (take)"] # [doc = " match set.entry(\"c\") {"] # [doc = "     Entry::Vacant(_) => unreachable!(),"] # [doc = "     Entry::Occupied(view) => {"] # [doc = "         assert_eq!(view.remove(), \"c\");"] # [doc = "     }"] # [doc = " }"] # [doc = " assert_eq!(set.get(&\"c\"), None);"] # [doc = " assert_eq!(set.len(), 2);"] # [doc = " ```"] # [unstable (feature = "hash_set_entry" , issue = "60896")] pub struct OccupiedEntry < 'a , T , S > { base : base :: OccupiedEntry < 'a , T , S > , }
};
}
