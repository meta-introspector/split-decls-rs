macro_rules! deps {
    () => {
        LinesWithTerminator!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl < 'a > Iterator for LinesWithTerminator < 'a > { type Item = & 'a str ; # [inline] fn next (& mut self) -> Option < & 'a str > { match self . data . find ('\n') { None if self . data . is_empty () => None , None => { let line = self . data ; self . data = "" ; Some (line) } Some (end) => { let line = & self . data [.. end + 1] ; self . data = & self . data [end + 1 ..] ; Some (line) } } } }
    };
}

impl_343!();