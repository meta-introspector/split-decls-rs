// Generated macro for string_roundtrip (function)
macro_rules! Depcrate_itemstring_roundtrip {
() => {
// Module: crate::item
// Provides: {"string_roundtrip"}
// Dependencies: {}
# [test] # [cfg (feature = "parse")] # [cfg (feature = "display")] fn string_roundtrip () { value ("hello") . to_string () . parse :: < Item > () . unwrap () ; }
};
}
