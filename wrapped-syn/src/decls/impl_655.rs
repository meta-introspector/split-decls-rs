macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_655 {
    () => {
        deps!();
        impl < 'a , T > DoubleEndedIterator for IterMut < 'a , T > { fn next_back (& mut self) -> Option < Self :: Item > { self . inner . next_back () } }
    };
}

impl_655!();