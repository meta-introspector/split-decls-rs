macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < A , I , const N : usize > Index < I > for Chunk < A , N > where I : SliceIndex < [A] > , { type Output = I :: Output ; fn index (& self , index : I) -> & Self :: Output { self . as_slice () . index (index) } }
    };
}

impl_57!();