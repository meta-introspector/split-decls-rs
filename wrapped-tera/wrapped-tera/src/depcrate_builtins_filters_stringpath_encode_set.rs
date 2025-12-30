// Generated macro for PATH_ENCODE_SET (const)
macro_rules! Depcrate_builtins_filters_stringPATH_ENCODE_SET {
() => {
// Module: crate::builtins::filters::string
// Provides: {"PATH_ENCODE_SET"}
// Dependencies: {}
# [doc = " https://url.spec.whatwg.org/#path-percent-encode-set"] # [cfg (feature = "urlencode")] const PATH_ENCODE_SET : & AsciiSet = & FRAGMENT_ENCODE_SET . add (b'#') . add (b'?') . add (b'{') . add (b'}') ;
};
}
