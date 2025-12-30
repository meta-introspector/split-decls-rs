// Generated macro for encodings (function)
macro_rules! Depcrate_content_encodingencodings {
() => {
// Module: crate::content_encoding
// Provides: {"encodings"}
// Dependencies: {}
# [cfg (any (feature = "compression-gzip" , feature = "compression-br" , feature = "compression-zstd" , feature = "compression-deflate" , feature = "fs" ,))] pub (crate) fn encodings < 'a > (headers : & 'a http :: HeaderMap , supported_encoding : impl SupportedEncodings + 'a ,) -> impl Iterator < Item = (Encoding , QValue) > + 'a { headers . get_all (http :: header :: ACCEPT_ENCODING) . iter () . filter_map (| hval | hval . to_str () . ok ()) . flat_map (| s | s . split (',')) . filter_map (move | v | { let mut v = v . splitn (2 , ';') ; let encoding = match Encoding :: parse (v . next () . unwrap () . trim () , supported_encoding) { Some (encoding) => encoding , None => return None , } ; let qval = if let Some (qval) = v . next () { QValue :: parse (qval . trim ()) ? } else { QValue :: one () } ; Some ((encoding , qval)) }) }
};
}
