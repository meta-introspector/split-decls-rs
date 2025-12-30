// Generated macro for impl_80 (impl)
macro_rules! Depcrate_hstring_builderimpl_80 {
() => {
// Module: crate::hstring_builder
// Provides: {"impl_80"}
// Dependencies: {}
impl HStringBuilder { # [doc = " Creates a preallocated `HSTRING` value."] pub fn new (len : usize) -> Self { let header = HStringHeader :: alloc (len . try_into () . unwrap ()) ; if len > 0 { unsafe { core :: ptr :: write_bytes ((* header) . data , 0 , len) } ; } Self (header) } # [doc = " Shortens the string by removing any trailing 0 characters."] pub fn trim_end (& mut self) { if let Some (header) = self . as_header_mut () { while header . len > 0 && unsafe { header . data . offset (header . len as isize - 1) . read () == 0 } { header . len -= 1 ; } if header . len == 0 { unsafe { HStringHeader :: free (self . 0) ; } self . 0 = core :: ptr :: null_mut () ; } } } # [doc = " Allows the `HSTRING` to be constructed from bytes."] pub fn as_bytes_mut (& mut self) -> & mut [u8] { if let Some (header) = self . as_header () { unsafe { core :: slice :: from_raw_parts_mut (header . data as * mut _ , header . len as usize * 2) } } else { & mut [] } } fn as_header (& self) -> Option < & HStringHeader > { unsafe { self . 0 . as_ref () } } fn as_header_mut (& mut self) -> Option < & mut HStringHeader > { unsafe { self . 0 . as_mut () } } }
};
}
