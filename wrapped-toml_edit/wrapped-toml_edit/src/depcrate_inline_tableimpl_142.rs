// Generated macro for impl_142 (impl)
macro_rules! Depcrate_inline_tableimpl_142 {
() => {
// Module: crate::inline_table
// Provides: {"impl_142"}
// Dependencies: {}
impl < 'a > InlineVacantEntry < 'a > { # [doc = " Gets a reference to the entry key"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use toml_edit::Table;"] # [doc = ""] # [doc = " let mut map = Table::new();"] # [doc = ""] # [doc = " assert_eq!(\"foo\", map.entry(\"foo\").key());"] # [doc = " ```"] pub fn key (& self) -> & str { self . entry . key () . get () } # [doc = " Sets the value of the entry with the `VacantEntry`'s key,"] # [doc = " and returns a mutable reference to it"] pub fn insert (self , value : Value) -> & 'a mut Value { let entry = self . entry ; let value = Item :: Value (value) ; entry . insert (value) . as_value_mut () . unwrap () } }
};
}
