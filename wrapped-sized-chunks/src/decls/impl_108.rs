macro_rules! deps {
    () => {
        Drain!();
        SparseChunk!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < A , const N : usize > IntoIterator for SparseChunk < A , N > where BitsImpl < N > : Bits , { type Item = A ; type IntoIter = Drain < A , N > ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . drain () } }
    };
}

impl_108!()