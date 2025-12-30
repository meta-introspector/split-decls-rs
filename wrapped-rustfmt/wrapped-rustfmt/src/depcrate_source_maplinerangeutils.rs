// Generated macro for LineRangeUtils (trait)
macro_rules! Depcrate_source_mapLineRangeUtils {
() => {
// Module: crate::source_map
// Provides: {"LineRangeUtils"}
// Dependencies: {}
pub (crate) trait LineRangeUtils { # [doc = " Returns the `LineRange` that corresponds to `span` in `self`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `span` crosses a file boundary, which shouldn't happen."] fn lookup_line_range (& self , span : Span) -> LineRange ; }
};
}
