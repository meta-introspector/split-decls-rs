// Generated macro for string_roundtrip (function)
macro_rules! Depcrate_valuestring_roundtrip {
() => {
// Module: crate::value
// Provides: {"string_roundtrip"}
// Dependencies: {}
# [test] # [cfg (feature = "parse")] # [cfg (feature = "display")] fn string_roundtrip () { Value :: from ("hello") . to_string () . parse :: < Value > () . unwrap () ; }
};
}
