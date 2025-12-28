macro_rules! deps {
    () => {
        SparseChunk!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < A , const N : usize > Eq for SparseChunk < A , N > where A : Eq , BitsImpl < N > : Bits , { }
    };
}

impl_113!()