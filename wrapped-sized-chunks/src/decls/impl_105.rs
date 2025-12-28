macro_rules! deps {
    () => {
        SparseChunk!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < A , const N : usize > Default for SparseChunk < A , N > where BitsImpl < N > : Bits , { fn default () -> Self { Self :: new () } }
    };
}

impl_105!()