macro_rules! deps {
    () => {
        OwnedIter!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < A , const N : usize > DoubleEndedIterator for OwnedIter < A , N > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . buffer . pop_back () } }
    };
}

impl_163!();