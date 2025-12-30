// Generated macro for impl_28 (impl)
macro_rules! Depcrate_hstringimpl_28 {
() => {
// Module: crate::hstring
// Provides: {"impl_28"}
// Dependencies: {}
impl HSTRING { # [doc = " Create an empty `HSTRING`."] # [doc = ""] # [doc = " This function does not allocate memory."] pub const fn new () -> Self { Self (core :: ptr :: null_mut ()) } # [doc = " Create a `HSTRING` from a slice of 16 bit characters (wchars)."] pub fn from_wide (value : & [u16]) -> Self { unsafe { Self :: from_wide_iter (value . iter () . copied () , value . len ()) } } # [doc = " Get the contents of this `HSTRING` as a String lossily."] pub fn to_string_lossy (& self) -> String { String :: from_utf16_lossy (self) } # [doc = " Get the contents of this `HSTRING` as a OsString."] # [cfg (feature = "std")] pub fn to_os_string (& self) -> std :: ffi :: OsString { std :: os :: windows :: ffi :: OsStringExt :: from_wide (self) } # [doc = " Allow this string to be displayed."] pub fn display (& self) -> impl core :: fmt :: Display + '_ { Decode (move | | core :: char :: decode_utf16 (self . iter () . cloned ())) } # [doc = " # Safety"] # [doc = " len must not be less than the number of items in the iterator."] unsafe fn from_wide_iter < I : Iterator < Item = u16 > > (iter : I , len : usize) -> Self { if len == 0 { return Self :: new () ; } let ptr = HStringHeader :: alloc (len . try_into () . unwrap ()) ; for (index , wide) in iter . enumerate () { debug_assert ! (index < len) ; unsafe { (* ptr) . data . add (index) . write (wide) ; (* ptr) . len = index as u32 + 1 ; } } unsafe { (* ptr) . data . offset ((* ptr) . len as isize) . write (0) ; } Self (ptr) } fn as_header (& self) -> Option < & HStringHeader > { unsafe { self . 0 . as_ref () } } }
};
}
