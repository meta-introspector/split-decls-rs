// Generated macro for impl_126 (impl)
macro_rules! Depcrate_pcstrimpl_126 {
() => {
// Module: crate::pcstr
// Provides: {"impl_126"}
// Dependencies: {}
impl PCSTR { # [doc = " Construct a new `PCSTR` from a raw pointer"] pub const fn from_raw (ptr : * const u8) -> Self { Self (ptr) } # [doc = " Construct a null `PCSTR`"] pub const fn null () -> Self { Self (core :: ptr :: null ()) } # [doc = " Returns a raw pointer to the `PCSTR`"] pub const fn as_ptr (& self) -> * const u8 { self . 0 } # [doc = " Checks whether the `PCSTR` is null"] pub fn is_null (& self) -> bool { self . 0 . is_null () } # [doc = " String data without the trailing 0"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `PCSTR`'s pointer needs to be valid for reads up until and including the next `\\0`."] pub unsafe fn as_bytes (& self) -> & [u8] { unsafe { let len = strlen (* self) ; core :: slice :: from_raw_parts (self . 0 , len) } } # [doc = " Copy the `PCSTR` into a Rust `String`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See the safety information for `PCSTR::as_bytes`."] pub unsafe fn to_string (& self) -> core :: result :: Result < String , alloc :: string :: FromUtf8Error > { unsafe { String :: from_utf8 (self . as_bytes () . into ()) } } # [doc = " Allow this string to be displayed."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See the safety information for `PCSTR::as_bytes`."] pub unsafe fn display (& self) -> impl core :: fmt :: Display + '_ { unsafe { Decode (move | | decode_utf8 (self . as_bytes ())) } } }
};
}
