// Generated macro for default_roundtrip (function)
macro_rules! Depcrate_documentdefault_roundtrip {
() => {
// Module: crate::document
// Provides: {"default_roundtrip"}
// Dependencies: {}
# [test] # [cfg (feature = "parse")] # [cfg (feature = "display")] fn default_roundtrip () { DocumentMut :: default () . to_string () . parse :: < DocumentMut > () . unwrap () ; }
};
}
