// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'a > Iterator for Utf16Chars < 'a > { type Item = char ; # [inline (always)] fn next (& mut self) -> Option < char > { let (& first , tail) = self . remaining . split_first () ? ; self . remaining = tail ; let surrogate_base = first . wrapping_sub (0xD800) ; if surrogate_base > (0xDFFF - 0xD800) { return Some (unsafe { char :: from_u32_unchecked (u32 :: from (first)) }) ; } Some (self . surrogate_next (surrogate_base , first)) } }
};
}
