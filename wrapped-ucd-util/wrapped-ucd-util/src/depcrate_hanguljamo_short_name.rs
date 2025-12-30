// Generated macro for jamo_short_name (function)
macro_rules! Depcrate_hanguljamo_short_name {
() => {
// Module: crate::hangul
// Provides: {"jamo_short_name"}
// Dependencies: {}
fn jamo_short_name < 'a > (table : JamoShortName < 'a > , cp : u32) -> & 'a str { let i = table . binary_search_by_key (& cp , | p | p . 0) . unwrap () ; table [i] . 1 }
};
}
