macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_642 {
    () => {
        deps!();
        impl < 'a , T > DoubleEndedIterator for Iter < 'a , T > { fn next_back (& mut self) -> Option < Self :: Item > { self . inner . next_back () } }
    };
}

impl_642!();