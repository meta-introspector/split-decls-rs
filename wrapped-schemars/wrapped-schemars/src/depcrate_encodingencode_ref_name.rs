// Generated macro for encode_ref_name (function)
macro_rules! Depcrate_encodingencode_ref_name {
() => {
// Module: crate::encoding
// Provides: {"encode_ref_name"}
// Dependencies: {}
# [doc = " Encodes a string for insertion into a JSON Pointer in URI fragment representation."] # [must_use] pub fn encode_ref_name (name : & str) -> Cow < '_ , str > { fn needs_encoding (byte : u8) -> bool { match byte { b'~' | b'/' => true , b'!' | b'$' | b'&' ..= b';' | b'=' | b'?' ..= b'Z' | b'_' | b'a' ..= b'z' => false , _ => true , } } if name . bytes () . any (needs_encoding) { let mut buf = String :: new () ; for byte in name . bytes () { if byte == b'~' { buf . push_str ("~0") ; } else if byte == b'/' { buf . push_str ("~1") ; } else if needs_encoding (byte) { write ! (buf , "%{byte:2X}") . unwrap () ; } else { buf . push (byte as char) ; } } Cow :: Owned (buf) } else { Cow :: Borrowed (name) } }
};
}
