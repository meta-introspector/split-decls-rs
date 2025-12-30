// Generated macro for impl_332 (impl)
macro_rules! Depcrate_tableimpl_332 {
() => {
// Module: crate::table
// Provides: {"impl_332"}
// Dependencies: {}
impl < 'a > Entry < 'a > { # [doc = " Returns the entry key"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use toml_edit::Table;"] # [doc = ""] # [doc = " let mut map = Table::new();"] # [doc = ""] # [doc = " assert_eq!(\"hello\", map.entry(\"hello\").key());"] # [doc = " ```"] pub fn key (& self) -> & str { match self { Entry :: Occupied (e) => e . key () , Entry :: Vacant (e) => e . key () , } } # [doc = " Ensures a value is in the entry by inserting the default if empty, and returns"] # [doc = " a mutable reference to the value in the entry."] pub fn or_insert (self , default : Item) -> & 'a mut Item { match self { Entry :: Occupied (entry) => entry . into_mut () , Entry :: Vacant (entry) => entry . insert (default) , } } # [doc = " Ensures a value is in the entry by inserting the result of the default function if empty,"] # [doc = " and returns a mutable reference to the value in the entry."] pub fn or_insert_with < F : FnOnce () -> Item > (self , default : F) -> & 'a mut Item { match self { Entry :: Occupied (entry) => entry . into_mut () , Entry :: Vacant (entry) => entry . insert (default ()) , } } }
};
}
