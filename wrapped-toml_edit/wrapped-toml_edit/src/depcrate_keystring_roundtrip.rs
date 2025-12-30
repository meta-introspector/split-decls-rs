// Generated macro for string_roundtrip (function)
macro_rules! Depcrate_keystring_roundtrip {
() => {
// Module: crate::key
// Provides: {"string_roundtrip"}
// Dependencies: {}
# [test] # [cfg (feature = "parse")] # [cfg (feature = "display")] fn string_roundtrip () { Key :: new ("hello") . to_string () . parse :: < Key > () . unwrap () ; }
};
}
