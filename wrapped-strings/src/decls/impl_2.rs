macro_rules! deps {
    () => {
        BSTR!();
        Decode!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl BSTR { # [doc = " Create an empty `BSTR`."] # [doc = ""] # [doc = " This function does not allocate memory."] pub const fn new () -> Self { Self (core :: ptr :: null_mut ()) } # [doc = " Create a `BSTR` from a slice of 16 bit characters (wchars)."] pub fn from_wide (value : & [u16]) -> Self { if value . is_empty () { return Self :: new () ; } let result = unsafe { Self (bindings :: SysAllocStringLen (value . as_ptr () , value . len () . try_into () . unwrap () ,)) } ; if result . is_empty () { panic ! ("allocation failed") ; } result } # [doc = " Allow this string to be displayed."] pub fn display (& self) -> impl core :: fmt :: Display + '_ { Decode (move | | core :: char :: decode_utf16 (self . iter () . cloned ())) } # [doc = " # Safety"] # [doc (hidden)] pub unsafe fn from_raw (raw : * const u16) -> Self { Self (raw) } # [doc = " # Safety"] # [doc (hidden)] pub fn into_raw (self) -> * const u16 { unsafe { core :: mem :: transmute (self) } } }
    };
}

impl_2!();