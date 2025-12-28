macro_rules! deps {
    () => {
        SparseChunk!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < A , const N : usize > IndexMut < usize > for SparseChunk < A , N > where BitsImpl < N > : Bits , { # [inline] fn index_mut (& mut self , index : usize) -> & mut Self :: Output { self . get_mut (index) . unwrap () } }
    };
}

impl_107!();