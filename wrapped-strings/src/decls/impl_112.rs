macro_rules! deps {
    () => {
        HSTRING!();
        Decode!();
        PCWSTR!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl PCWSTR { # [doc = " Construct a new `PCWSTR` from a raw pointer"] pub const fn from_raw (ptr : * const u16) -> Self { Self (ptr) } # [doc = " Construct a null `PCWSTR`"] pub const fn null () -> Self { Self (core :: ptr :: null ()) } # [doc = " Returns a raw pointer to the `PCWSTR`"] pub const fn as_ptr (& self) -> * const u16 { self . 0 } # [doc = " Checks whether the `PCWSTR` is null"] pub fn is_null (& self) -> bool { self . 0 . is_null () } # [doc = " String length without the trailing 0"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `PCWSTR`'s pointer needs to be valid for reads up until and including the next `\\0`."] pub unsafe fn len (& self) -> usize { unsafe extern "C" { fn wcslen (s : * const u16) -> usize ; } unsafe { wcslen (self . 0) } } # [doc = " Returns `true` if the string length is zero, and `false` otherwise."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `PCWSTR`'s pointer needs to be valid for reads up until and including the next `\\0`."] pub unsafe fn is_empty (& self) -> bool { unsafe { self . len () == 0 } } # [doc = " String data without the trailing 0"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `PCWSTR`'s pointer needs to be valid for reads up until and including the next `\\0`."] pub unsafe fn as_wide (& self) -> & [u16] { unsafe { core :: slice :: from_raw_parts (self . 0 , self . len ()) } } # [doc = " Copy the `PCWSTR` into a Rust `String`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See the safety information for `PCWSTR::as_wide`."] pub unsafe fn to_string (& self) -> core :: result :: Result < String , alloc :: string :: FromUtf16Error > { unsafe { String :: from_utf16 (self . as_wide ()) } } # [doc = " Copy the `PCWSTR` into an `HSTRING`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See the safety information for `PCWSTR::as_wide`."] pub unsafe fn to_hstring (& self) -> HSTRING { unsafe { HSTRING :: from_wide (self . as_wide ()) } } # [doc = " Allow this string to be displayed."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See the safety information for `PCWSTR::as_wide`."] pub unsafe fn display (& self) -> impl core :: fmt :: Display + '_ { unsafe { Decode (move | | core :: char :: decode_utf16 (self . as_wide () . iter () . cloned ())) } } }
    };
}

impl_112!()