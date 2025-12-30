// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a > Utf16Chars < 'a > { # [inline (always)] # [doc = " Creates the iterator from a `u16` slice."] pub fn new (code_units : & 'a [u16]) -> Self { Utf16Chars :: < 'a > { remaining : code_units , } } # [doc = " Views the current remaining data in the iterator as a subslice"] # [doc = " of the original slice."] # [inline (always)] pub fn as_slice (& self) -> & 'a [u16] { self . remaining } # [inline (never)] fn surrogate_next (& mut self , surrogate_base : u16 , first : u16) -> char { if surrogate_base <= (0xDBFF - 0xD800) { if let Some ((& low , tail_tail)) = self . remaining . split_first () { if in_inclusive_range16 (low , 0xDC00 , 0xDFFF) { self . remaining = tail_tail ; return unsafe { char :: from_u32_unchecked ((u32 :: from (first) << 10) + u32 :: from (low) - (((0xD800u32 << 10) - 0x10000u32) + 0xDC00u32) ,) } ; } } } '\u{FFFD}' } # [inline (never)] fn surrogate_next_back (& mut self , last : u16) -> char { if in_inclusive_range16 (last , 0xDC00 , 0xDFFF) { if let Some ((& high , head_head)) = self . remaining . split_last () { if in_inclusive_range16 (high , 0xD800 , 0xDBFF) { self . remaining = head_head ; return unsafe { char :: from_u32_unchecked ((u32 :: from (high) << 10) + u32 :: from (last) - (((0xD800u32 << 10) - 0x10000u32) + 0xDC00u32) ,) } ; } } } '\u{FFFD}' } }
};
}
