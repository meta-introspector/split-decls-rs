// Generated macro for encode_braced (function)
macro_rules! Depcrate_fmtencode_braced {
() => {
// Module: crate::fmt
// Provides: {"encode_braced"}
// Dependencies: {}
# [inline] fn encode_braced < 'b > (src : & [u8 ; 16] , buffer : & 'b mut [u8] , upper : bool) -> & 'b mut str { let buf = & mut buffer [.. Hyphenated :: LENGTH + 2] ; let buf : & mut [u8 ; Hyphenated :: LENGTH + 2] = buf . try_into () . unwrap () ; # [cfg_attr (all (uuid_unstable , feature = "zerocopy") , derive (zerocopy :: IntoBytes))] # [repr (C)] struct Braced { open_curly : u8 , hyphenated : [u8 ; Hyphenated :: LENGTH] , close_curly : u8 , } let braced = Braced { open_curly : b'{' , hyphenated : format_hyphenated (src , upper) , close_curly : b'}' , } ; * buf = unsafe_transmute ! (braced) ; unsafe { str :: from_utf8_unchecked_mut (buf) } }
};
}
