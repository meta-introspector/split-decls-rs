// Generated macro for impl_18 (impl)
macro_rules! Depcrate_reportimpl_18 {
() => {
// Module: crate::report
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for ErrorReportingUtf16Chars < 'a > { # [inline (always)] fn next_back (& mut self) -> Option < Result < char , Utf16CharsError > > { let (& last , head) = self . remaining . split_last () ? ; self . remaining = head ; if ! in_inclusive_range16 (last , 0xD800 , 0xDFFF) { return Some (Ok (unsafe { char :: from_u32_unchecked (u32 :: from (last)) })) ; } Some (self . surrogate_next_back (last)) } }
};
}
