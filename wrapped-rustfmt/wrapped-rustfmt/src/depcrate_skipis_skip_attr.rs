// Generated macro for is_skip_attr (function)
macro_rules! Depcrate_skipis_skip_attr {
() => {
// Module: crate::skip
// Provides: {"is_skip_attr"}
// Dependencies: {}
# [doc = " Say if you're playing with `rustfmt`'s skip attribute"] pub (crate) fn is_skip_attr (segments : & [ast :: PathSegment]) -> bool { if segments . len () < 2 || segments [0] . ident . to_string () != RUSTFMT { return false ; } match segments . len () { 2 => segments [1] . ident . to_string () == SKIP , 3 => { segments [1] . ident . to_string () == SKIP && ["macros" , "attributes"] . iter () . any (| & n | n == pprust :: path_segment_to_string (& segments [2])) } _ => false , } }
};
}
