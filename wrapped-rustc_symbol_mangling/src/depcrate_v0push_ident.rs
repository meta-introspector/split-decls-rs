// Generated macro for push_ident (function)
macro_rules! Depcrate_v0push_ident {
() => {
// Module: crate::v0
// Provides: {"push_ident"}
// Dependencies: {}
pub (crate) fn push_ident (ident : & str , output : & mut String) { let mut use_punycode = false ; for b in ident . bytes () { match b { b'_' | b'a' ..= b'z' | b'A' ..= b'Z' | b'0' ..= b'9' => { } 0x80 ..= 0xff => use_punycode = true , _ => bug ! ("symbol_names: bad byte {} in ident {:?}" , b , ident) , } } let punycode_string ; let ident = if use_punycode { output . push ('u') ; let mut punycode_bytes = match punycode :: encode (ident) { Ok (s) => s . into_bytes () , Err (()) => bug ! ("symbol_names: punycode encoding failed for ident {:?}" , ident) , } ; if let Some (c) = punycode_bytes . iter_mut () . rfind (| & & mut c | c == b'-') { * c = b'_' ; } punycode_string = String :: from_utf8 (punycode_bytes) . unwrap () ; & punycode_string } else { ident } ; let _ = write ! (output , "{}" , ident . len ()) ; if let Some ('_' | '0' ..= '9') = ident . chars () . next () { output . push ('_') ; } output . push_str (ident) ; }
};
}
