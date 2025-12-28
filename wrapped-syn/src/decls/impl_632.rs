macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_632 {
    () => {
        deps!();
        impl < T > DoubleEndedIterator for IntoIter < T > { fn next_back (& mut self) -> Option < Self :: Item > { self . inner . next_back () } }
    };
}

impl_632!();