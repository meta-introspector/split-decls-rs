// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Utf8Chars < 'a > { # [inline] fn next_back (& mut self) -> Option < char > { if self . remaining . is_empty () { return None ; } let mut attempt = 1 ; for b in self . remaining . iter () . rev () { if b & 0xC0 != 0x80 { let (head , tail) = self . remaining . split_at (self . remaining . len () - attempt) ; let mut inner = Utf8Chars :: new (tail) ; let candidate = inner . next () ; if inner . as_slice () . is_empty () { self . remaining = head ; return candidate ; } break ; } if attempt == 4 { break ; } attempt += 1 ; } self . remaining = & self . remaining [.. self . remaining . len () - 1] ; Some ('\u{FFFD}') } }
};
}
