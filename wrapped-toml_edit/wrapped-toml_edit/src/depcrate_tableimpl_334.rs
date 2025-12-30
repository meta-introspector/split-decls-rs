// Generated macro for impl_334 (impl)
macro_rules! Depcrate_tableimpl_334 {
() => {
// Module: crate::table
// Provides: {"impl_334"}
// Dependencies: {}
impl < 'a > OccupiedEntry < 'a > { # [doc = " Gets a reference to the entry key"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use toml_edit::Table;"] # [doc = ""] # [doc = " let mut map = Table::new();"] # [doc = ""] # [doc = " assert_eq!(\"foo\", map.entry(\"foo\").key());"] # [doc = " ```"] pub fn key (& self) -> & str { self . entry . key () . get () } # [doc = " Gets a mutable reference to the entry key"] pub fn key_mut (& mut self) -> KeyMut < '_ > { use indexmap :: map :: MutableEntryKey ; self . entry . key_mut () . as_mut () } # [doc = " Gets a reference to the value in the entry."] pub fn get (& self) -> & Item { self . entry . get () } # [doc = " Gets a mutable reference to the value in the entry."] pub fn get_mut (& mut self) -> & mut Item { self . entry . get_mut () } # [doc = " Converts the `OccupiedEntry` into a mutable reference to the value in the entry"] # [doc = " with a lifetime bound to the map itself"] pub fn into_mut (self) -> & 'a mut Item { self . entry . into_mut () } # [doc = " Sets the value of the entry, and returns the entry's old value"] pub fn insert (& mut self , value : Item) -> Item { self . entry . insert (value) } # [doc = " Takes the value out of the entry, and returns it"] pub fn remove (self) -> Item { self . entry . shift_remove () } }
};
}
