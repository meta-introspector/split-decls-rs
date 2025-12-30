// Generated macro for impl_1144 (impl)
macro_rules! Depcrate_parse_sessionimpl_1144 {
() => {
// Module: crate::parse::session
// Provides: {"impl_1144"}
// Dependencies: {}
impl LineRangeUtils for ParseSess { fn lookup_line_range (& self , span : Span) -> LineRange { let snippet = self . raw_psess . source_map () . span_to_snippet (span) . unwrap_or_default () ; let lo = self . raw_psess . source_map () . lookup_line (span . lo ()) . unwrap () ; let hi = self . raw_psess . source_map () . lookup_line (span . hi ()) . unwrap () ; debug_assert_eq ! (lo . sf . name , hi . sf . name , "span crossed file boundary: lo: {lo:?}, hi: {hi:?}") ; let offset = 1 + if starts_with_newline (& snippet) { 1 } else { 0 } ; LineRange { file : lo . sf . clone () , lo : lo . line + offset , hi : hi . line + offset , } } }
};
}
