// Generated macro for impl_19 (impl)
macro_rules! Depcrate_reportimpl_19 {
() => {
// Module: crate::report
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for ErrorReportingUtf8Chars < 'a > { # [inline] fn next_back (& mut self) -> Option < Result < char , Utf8CharsError > > { if self . remaining . is_empty () { return None ; } let mut attempt = 1 ; for b in self . remaining . iter () . rev () { if b & 0xC0 != 0x80 { let (head , tail) = self . remaining . split_at (self . remaining . len () - attempt) ; let mut inner = ErrorReportingUtf8Chars :: new (tail) ; let candidate = inner . next () ; if inner . as_slice () . is_empty () { self . remaining = head ; return candidate ; } break ; } if attempt == 4 { break ; } attempt += 1 ; } self . remaining = & self . remaining [.. self . remaining . len () - 1] ; Some (Err (Utf8CharsError)) } }
};
}
