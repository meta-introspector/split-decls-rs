// Generated macro for impl_336 (impl)
macro_rules! Depcrate_tableimpl_336 {
() => {
// Module: crate::table
// Provides: {"impl_336"}
// Dependencies: {}
impl < 'a > VacantEntry < 'a > { # [doc = " Gets a reference to the entry key"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use toml_edit::Table;"] # [doc = ""] # [doc = " let mut map = Table::new();"] # [doc = ""] # [doc = " assert_eq!(\"foo\", map.entry(\"foo\").key());"] # [doc = " ```"] pub fn key (& self) -> & str { self . entry . key () . get () } # [doc = " Sets the value of the entry with the `VacantEntry`'s key,"] # [doc = " and returns a mutable reference to it"] pub fn insert (self , value : Item) -> & 'a mut Item { let entry = self . entry ; entry . insert (value) } }
};
}
