macro_rules! deps {
    () => {
        Graphemes!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for Graphemes < 'a > { # [inline] fn next_back (& mut self) -> Option < & 'a str > { let end = self . cursor_back . cur_cursor () ; if end == self . cursor . cur_cursor () { return None ; } let prev = self . cursor_back . prev_boundary (self . string , 0) . unwrap () . unwrap () ; Some (& self . string [prev .. end]) } }
    };
}

impl_8!();