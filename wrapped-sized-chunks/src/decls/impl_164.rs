macro_rules! deps {
    () => {
        OwnedIter!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < A , const N : usize > ExactSizeIterator for OwnedIter < A , N > { }
    };
}

impl_164!();