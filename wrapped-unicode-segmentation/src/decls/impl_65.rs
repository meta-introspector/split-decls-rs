macro_rules! deps {
    () => {
        AsciiWordBoundIter!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for AsciiWordBoundIter < 'a > { fn next_back (& mut self) -> Option < (usize , & 'a str) > { let rest = self . rest ; if rest . is_empty () { return None ; } let bytes = rest . as_bytes () ; let len = bytes . len () ; if bytes [len - 1] == b' ' { let mut start = len - 1 ; while start > 0 && bytes [start - 1] == b' ' { start -= 1 ; } let word = & rest [start ..] ; let pos = self . offset + start ; self . rest = & rest [.. start] ; return Some ((pos , word)) ; } if Self :: is_core (bytes [len - 1]) { let mut start = len - 1 ; while start > 0 { let b = bytes [start - 1] ; let prev = if start >= 2 { bytes [start - 2] } else { b } ; let next = bytes [start] ; if Self :: is_core (b) || Self :: is_infix (b , prev , next) { start -= 1 ; } else { break ; } } let word = & rest [start ..] ; let pos = self . offset + start ; self . rest = & rest [.. start] ; return Some ((pos , word)) ; } if len >= 2 && bytes [len - 2] == b'\r' && bytes [len - 1] == b'\n' { let start = len - 2 ; let word = & rest [start ..] ; let pos = self . offset + start ; self . rest = & rest [.. start] ; return Some ((pos , word)) ; } let start = len - 1 ; let word = & rest [start ..] ; let pos = self . offset + start ; self . rest = & rest [.. start] ; Some ((pos , word)) } }
    };
}

impl_65!();