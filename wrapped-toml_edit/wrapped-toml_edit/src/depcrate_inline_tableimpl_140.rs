// Generated macro for impl_140 (impl)
macro_rules! Depcrate_inline_tableimpl_140 {
() => {
// Module: crate::inline_table
// Provides: {"impl_140"}
// Dependencies: {}
impl < 'a > InlineOccupiedEntry < 'a > { # [doc = " Gets a reference to the entry key"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use toml_edit::Table;"] # [doc = ""] # [doc = " let mut map = Table::new();"] # [doc = ""] # [doc = " assert_eq!(\"foo\", map.entry(\"foo\").key());"] # [doc = " ```"] pub fn key (& self) -> & str { self . entry . key () . get () } # [doc = " Gets a mutable reference to the entry key"] pub fn key_mut (& mut self) -> KeyMut < '_ > { use indexmap :: map :: MutableEntryKey ; self . entry . key_mut () . as_mut () } # [doc = " Gets a reference to the value in the entry."] pub fn get (& self) -> & Value { self . entry . get () . as_value () . unwrap () } # [doc = " Gets a mutable reference to the value in the entry."] pub fn get_mut (& mut self) -> & mut Value { self . entry . get_mut () . as_value_mut () . unwrap () } # [doc = " Converts the `OccupiedEntry` into a mutable reference to the value in the entry"] # [doc = " with a lifetime bound to the map itself"] pub fn into_mut (self) -> & 'a mut Value { self . entry . into_mut () . as_value_mut () . unwrap () } # [doc = " Sets the value of the entry, and returns the entry's old value"] pub fn insert (& mut self , value : Value) -> Value { let value = Item :: Value (value) ; self . entry . insert (value) . into_value () . unwrap () } # [doc = " Takes the value out of the entry, and returns it"] pub fn remove (self) -> Value { self . entry . shift_remove () . into_value () . unwrap () } }
};
}
