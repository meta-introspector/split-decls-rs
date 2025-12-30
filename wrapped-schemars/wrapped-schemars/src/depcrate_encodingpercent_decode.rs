// Generated macro for percent_decode (function)
macro_rules! Depcrate_encodingpercent_decode {
() => {
// Module: crate::encoding
// Provides: {"percent_decode"}
// Dependencies: {}
# [doc = " Percent-decodes the given string, returning `None` if it results in invalid UTF-8."] # [doc = " A `%` that is not followed by two hex digits is treated as a literal `%`."] # [must_use] pub fn percent_decode (s : & str) -> Option < Cow < '_ , str > > { if s . contains ('%') { let mut buf = Vec :: < u8 > :: new () ; let mut segments = s . split ('%') ; buf . extend (segments . next () . unwrap_or_default () . as_bytes ()) ; for segment in segments { if let Some (decoded_byte) = segment . get (0 .. 2) . and_then (| p | u8 :: from_str_radix (p , 16) . ok ()) { buf . push (decoded_byte) ; buf . extend (& segment . as_bytes () [2 ..]) ; } else { buf . push (b'%') ; buf . extend (segment . as_bytes ()) ; } } String :: from_utf8 (buf) . ok () . map (Cow :: Owned) } else { Some (Cow :: Borrowed (s)) } }
};
}
