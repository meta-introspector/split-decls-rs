macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < A , T > DoubleEndedIterator for Iter < A , T > { fn next_back (& mut self) -> Option < Self :: Item > { self . array . pop () } }
    };
}

impl_2!();