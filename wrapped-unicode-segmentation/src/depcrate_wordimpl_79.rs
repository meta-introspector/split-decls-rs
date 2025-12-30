// Generated macro for impl_79 (impl)
macro_rules! Depcrate_wordimpl_79 {
() => {
// Module: crate::word
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'a > Iterator for AsciiWordBoundIter < 'a > { type Item = (usize , & 'a str) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if self . rest . is_empty () { return None ; } let bytes = self . rest . as_bytes () ; let len = bytes . len () ; if bytes [0] == b' ' { let mut i = 1 ; while i < len && bytes [i] == b' ' { i += 1 ; } let word = & self . rest [.. i] ; let pos = self . offset ; self . rest = & self . rest [i ..] ; self . offset += i ; return Some ((pos , word)) ; } if Self :: is_core (bytes [0]) { let mut i = 1 ; while i < len { let b = bytes [i] ; if Self :: is_core (b) || (i + 1 < len && Self :: is_infix (b , bytes [i - 1] , bytes [i + 1])) { i += 1 ; } else { break ; } } let word = & self . rest [.. i] ; let pos = self . offset ; self . rest = & self . rest [i ..] ; self . offset += i ; return Some ((pos , word)) ; } if bytes [0] == b'\r' && len >= 2 && bytes [1] == b'\n' { let word = & self . rest [.. 2] ; let pos = self . offset ; self . rest = & self . rest [2 ..] ; self . offset += 2 ; Some ((pos , word)) } else { let word = & self . rest [.. 1] ; let pos = self . offset ; self . rest = & self . rest [1 ..] ; self . offset += 1 ; Some ((pos , word)) } } }
};
}
