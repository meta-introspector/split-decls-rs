macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < A , const N : usize > DoubleEndedIterator for Iter < A , N > { fn next_back (& mut self) -> Option < Self :: Item > { if self . chunk . is_empty () { None } else { Some (self . chunk . pop_back ()) } } }
    };
}

impl_39!();