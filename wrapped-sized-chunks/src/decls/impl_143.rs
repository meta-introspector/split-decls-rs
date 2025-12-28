macro_rules! deps {
    () => {
        IndexIter!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < const N : usize > FusedIterator for IndexIter < N > { }
    };
}

impl_143!();