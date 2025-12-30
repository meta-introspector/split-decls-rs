// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Utf16Chars < 'a > { # [inline (always)] fn next_back (& mut self) -> Option < char > { let (& last , head) = self . remaining . split_last () ? ; self . remaining = head ; if ! in_inclusive_range16 (last , 0xD800 , 0xDFFF) { return Some (unsafe { char :: from_u32_unchecked (u32 :: from (last)) }) ; } Some (self . surrogate_next_back (last)) } }
};
}
