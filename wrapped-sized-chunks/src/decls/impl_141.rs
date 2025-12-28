macro_rules! deps {
    () => {
        IndexIter!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < const N : usize > DoubleEndedIterator for IndexIter < N > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { if self . remaining > 0 { self . remaining -= 1 ; Some (self . right_index . dec ()) } else { None } } }
    };
}

impl_141!();