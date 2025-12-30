// Generated macro for impl_324 (impl)
macro_rules! Depcrate_unicode_dataimpl_324 {
() => {
// Module: crate::unicode_data
// Provides: {"impl_324"}
// Dependencies: {}
impl UnicodeData { # [doc = " Returns true if and only if this record corresponds to the start of a"] # [doc = " range."] pub fn is_range_start (& self) -> bool { self . name . starts_with ('<') && self . name . ends_with ('>') && self . name . contains ("First") } # [doc = " Returns true if and only if this record corresponds to the end of a"] # [doc = " range."] pub fn is_range_end (& self) -> bool { self . name . starts_with ('<') && self . name . ends_with ('>') && self . name . contains ("Last") } }
};
}
