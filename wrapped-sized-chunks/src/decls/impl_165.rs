macro_rules! deps {
    () => {
        OwnedIter!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < A , const N : usize > FusedIterator for OwnedIter < A , N > { }
    };
}

impl_165!()