// Generated macro for impl_43 (impl)
macro_rules! Depcrate_documentimpl_43 {
() => {
// Module: crate::document
// Provides: {"impl_43"}
// Dependencies: {}
impl < S > Document < S > { # [doc = " Returns a reference to the root item."] pub fn as_item (& self) -> & Item { & self . root } # [doc = " Returns the root item."] pub fn into_item (self) -> Item { self . root } # [doc = " Returns a reference to the root table."] pub fn as_table (& self) -> & Table { self . root . as_table () . expect ("root should always be a table") } # [doc = " Returns the root table."] pub fn into_table (self) -> Table { self . root . into_table () . expect ("root should always be a table") } # [doc = " Returns an iterator over the root table."] pub fn iter (& self) -> Iter < '_ > { self . as_table () . iter () } # [doc = " Whitespace after last element"] pub fn trailing (& self) -> & RawString { & self . trailing } }
};
}
