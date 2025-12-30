// Generated macro for impl_60 (impl)
macro_rules! Depcrate_line_endingimpl_60 {
() => {
// Module: crate::line_ending
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a > Iterator for NonEmptyLines < 'a > { type Item = (& 'a str , Option < LineEnding >) ; fn next (& mut self) -> Option < Self :: Item > { while let Some (lf) = self . 0 . find ('\n') { if lf == 0 || (lf == 1 && self . 0 . as_bytes () [lf - 1] == b'\r') { self . 0 = & self . 0 [(lf + 1) ..] ; continue ; } let trimmed = match self . 0 . as_bytes () [lf - 1] { b'\r' => (& self . 0 [.. (lf - 1)] , Some (LineEnding :: CRLF)) , _ => (& self . 0 [.. lf] , Some (LineEnding :: LF)) , } ; self . 0 = & self . 0 [(lf + 1) ..] ; return Some (trimmed) ; } if self . 0 . is_empty () { None } else { let line = std :: mem :: take (& mut self . 0) ; Some ((line , None)) } } }
};
}
