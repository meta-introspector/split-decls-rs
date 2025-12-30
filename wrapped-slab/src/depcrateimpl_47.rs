// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a , T > VacantEntry < 'a , T > { # [doc = " Insert a value in the entry, returning a mutable reference to the value."] # [doc = ""] # [doc = " To get the key associated with the value, use `key` prior to calling"] # [doc = " `insert`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use slab::*;"] # [doc = " let mut slab = Slab::new();"] # [doc = ""] # [doc = " let hello = {"] # [doc = "     let entry = slab.vacant_entry();"] # [doc = "     let key = entry.key();"] # [doc = ""] # [doc = "     entry.insert((key, \"hello\"));"] # [doc = "     key"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(hello, slab[hello].0);"] # [doc = " assert_eq!(\"hello\", slab[hello].1);"] # [doc = " ```"] pub fn insert (self , val : T) -> & 'a mut T { self . slab . insert_at (self . key , val) ; match self . slab . entries . get_mut (self . key) { Some (& mut Entry :: Occupied (ref mut v)) => v , _ => unreachable ! () , } } # [doc = " Return the key associated with this entry."] # [doc = ""] # [doc = " A value stored in this entry will be associated with this key."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use slab::*;"] # [doc = " let mut slab = Slab::new();"] # [doc = ""] # [doc = " let hello = {"] # [doc = "     let entry = slab.vacant_entry();"] # [doc = "     let key = entry.key();"] # [doc = ""] # [doc = "     entry.insert((key, \"hello\"));"] # [doc = "     key"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(hello, slab[hello].0);"] # [doc = " assert_eq!(\"hello\", slab[hello].1);"] # [doc = " ```"] pub fn key (& self) -> usize { self . key } }
};
}
