// Generated macro for impl_427 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_427 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_427"}
// Dependencies: {}
impl < 'a , K , V : Default > Entry < 'a , K , V > { # [doc = " Ensures a value is in the entry by inserting the default value if empty,"] # [doc = " and returns a mutable reference to the value in the entry."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # fn main() {"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let mut map: HashMap<&str, Option<u32>> = HashMap::new();"] # [doc = " map.entry(\"poneyland\").or_default();"] # [doc = ""] # [doc = " assert_eq!(map[\"poneyland\"], None);"] # [doc = " # }"] # [doc = " ```"] # [inline] # [stable (feature = "entry_or_default" , since = "1.28.0")] pub fn or_default (self) -> & 'a mut V { match self { Occupied (entry) => entry . into_mut () , Vacant (entry) => entry . insert (Default :: default ()) , } } }
};
}
