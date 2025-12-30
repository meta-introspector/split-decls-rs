// Generated macro for impl_17 (impl)
macro_rules! Depcrate_reportimpl_17 {
() => {
// Module: crate::report
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'a > Iterator for ErrorReportingUtf16Chars < 'a > { type Item = Result < char , Utf16CharsError > ; # [inline (always)] fn next (& mut self) -> Option < Result < char , Utf16CharsError > > { let (& first , tail) = self . remaining . split_first () ? ; self . remaining = tail ; let surrogate_base = first . wrapping_sub (0xD800) ; if surrogate_base > (0xDFFF - 0xD800) { return Some (Ok (unsafe { char :: from_u32_unchecked (u32 :: from (first)) })) ; } Some (self . surrogate_next (surrogate_base , first)) } }
};
}
