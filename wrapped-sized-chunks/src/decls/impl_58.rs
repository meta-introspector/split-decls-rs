macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < A , I , const N : usize > IndexMut < I > for Chunk < A , N > where I : SliceIndex < [A] > , { fn index_mut (& mut self , index : I) -> & mut Self :: Output { self . as_mut_slice () . index_mut (index) } }
    };
}

impl_58!();