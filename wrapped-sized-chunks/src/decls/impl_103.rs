macro_rules! deps {
    () => {
        SparseChunk!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < A : Clone , const N : usize > Clone for SparseChunk < A , N > where BitsImpl < N > : Bits , { fn clone (& self) -> Self { let mut out = Self :: new () ; for index in & self . map { out . insert (index , self [index] . clone ()) ; } out } }
    };
}

impl_103!()