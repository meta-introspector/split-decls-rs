// Generated macro for USERINFO_ENCODE_SET (const)
macro_rules! Depcrate_builtins_filters_stringUSERINFO_ENCODE_SET {
() => {
// Module: crate::builtins::filters::string
// Provides: {"USERINFO_ENCODE_SET"}
// Dependencies: {}
# [doc = " https://url.spec.whatwg.org/#userinfo-percent-encode-set"] # [cfg (feature = "urlencode")] const USERINFO_ENCODE_SET : & AsciiSet = & PATH_ENCODE_SET . add (b'/') . add (b':') . add (b';') . add (b'=') . add (b'@') . add (b'[') . add (b'\\') . add (b']') . add (b'^') . add (b'|') ;
};
}
