macro_rules! deps {
    () => {
        IndexIter!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < const N : usize > ExactSizeIterator for IndexIter < N > { }
    };
}

impl_142!()