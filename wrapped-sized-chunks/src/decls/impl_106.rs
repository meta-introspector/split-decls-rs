macro_rules! deps {
    () => {
        SparseChunk!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < A , const N : usize > Index < usize > for SparseChunk < A , N > where BitsImpl < N > : Bits , { type Output = A ; # [inline] fn index (& self , index : usize) -> & Self :: Output { self . get (index) . unwrap () } }
    };
}

impl_106!()