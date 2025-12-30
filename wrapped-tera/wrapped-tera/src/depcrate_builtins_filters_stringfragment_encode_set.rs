// Generated macro for FRAGMENT_ENCODE_SET (const)
macro_rules! Depcrate_builtins_filters_stringFRAGMENT_ENCODE_SET {
() => {
// Module: crate::builtins::filters::string
// Provides: {"FRAGMENT_ENCODE_SET"}
// Dependencies: {}
# [doc = " https://url.spec.whatwg.org/#fragment-percent-encode-set"] # [cfg (feature = "urlencode")] const FRAGMENT_ENCODE_SET : & AsciiSet = & percent_encoding :: CONTROLS . add (b' ') . add (b'"') . add (b'<') . add (b'>') . add (b'`') ;
};
}
