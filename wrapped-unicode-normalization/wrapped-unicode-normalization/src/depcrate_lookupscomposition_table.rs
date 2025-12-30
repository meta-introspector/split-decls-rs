// Generated macro for composition_table (function)
macro_rules! Depcrate_lookupscomposition_table {
() => {
// Module: crate::lookups
// Provides: {"composition_table"}
// Dependencies: {}
pub (crate) fn composition_table (c1 : char , c2 : char) -> Option < char > { if c1 < '\u{10000}' && c2 < '\u{10000}' { mph_lookup ((c1 as u32) << 16 | (c2 as u32) , COMPOSITION_TABLE_SALT , COMPOSITION_TABLE_KV , pair_lookup_fk , pair_lookup_fv_opt , None ,) } else { composition_table_astral (c1 , c2) } }
};
}
