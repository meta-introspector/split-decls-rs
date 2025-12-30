// Generated macro for impl_138 (impl)
macro_rules! Depcrate_inline_tableimpl_138 {
() => {
// Module: crate::inline_table
// Provides: {"impl_138"}
// Dependencies: {}
impl < 'a > InlineEntry < 'a > { # [doc = " Returns the entry key"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use toml_edit::Table;"] # [doc = ""] # [doc = " let mut map = Table::new();"] # [doc = ""] # [doc = " assert_eq!(\"hello\", map.entry(\"hello\").key());"] # [doc = " ```"] pub fn key (& self) -> & str { match self { InlineEntry :: Occupied (e) => e . key () , InlineEntry :: Vacant (e) => e . key () , } } # [doc = " Ensures a value is in the entry by inserting the default if empty, and returns"] # [doc = " a mutable reference to the value in the entry."] pub fn or_insert (self , default : Value) -> & 'a mut Value { match self { InlineEntry :: Occupied (entry) => entry . into_mut () , InlineEntry :: Vacant (entry) => entry . insert (default) , } } # [doc = " Ensures a value is in the entry by inserting the result of the default function if empty,"] # [doc = " and returns a mutable reference to the value in the entry."] pub fn or_insert_with < F : FnOnce () -> Value > (self , default : F) -> & 'a mut Value { match self { InlineEntry :: Occupied (entry) => entry . into_mut () , InlineEntry :: Vacant (entry) => entry . insert (default ()) , } } }
};
}
