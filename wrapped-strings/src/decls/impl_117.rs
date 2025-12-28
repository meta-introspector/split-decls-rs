macro_rules! deps {
    () => {
        Decode!();
        PCSTR!();
        PSTR!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl PSTR { # [doc = " Construct a new `PSTR` from a raw pointer"] pub const fn from_raw (ptr : * mut u8) -> Self { Self (ptr) } # [doc = " Construct a null `PSTR`"] pub const fn null () -> Self { Self (core :: ptr :: null_mut ()) } # [doc = " Returns a raw pointer to the `PSTR`"] pub const fn as_ptr (& self) -> * mut u8 { self . 0 } # [doc = " Checks whether the `PSTR` is null"] pub fn is_null (& self) -> bool { self . 0 . is_null () } # [doc = " String data without the trailing 0"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `PSTR`'s pointer needs to be valid for reads up until and including the next `\\0`."] pub unsafe fn as_bytes (& self) -> & [u8] { unsafe { let len = strlen (PCSTR :: from_raw (self . 0)) ; core :: slice :: from_raw_parts (self . 0 , len) } } # [doc = " Copy the `PSTR` into a Rust `String`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See the safety information for `PSTR::as_bytes`."] pub unsafe fn to_string (& self) -> core :: result :: Result < String , alloc :: string :: FromUtf8Error > { unsafe { String :: from_utf8 (self . as_bytes () . into ()) } } # [doc = " Allow this string to be displayed."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See the safety information for `PSTR::as_bytes`."] pub unsafe fn display (& self) -> impl core :: fmt :: Display + '_ { unsafe { Decode (move | | decode_utf8 (self . as_bytes ())) } } }
    };
}

impl_117!()