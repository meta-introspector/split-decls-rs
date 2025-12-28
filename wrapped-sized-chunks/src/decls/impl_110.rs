macro_rules! deps {
    () => {
        SparseChunk!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < A , const N : usize > PartialEq for SparseChunk < A , N > where A : PartialEq , BitsImpl < N > : Bits , { fn eq (& self , other : & Self) -> bool { if self . map != other . map { return false ; } for index in self . indices () { if self . get (index) != other . get (index) { return false ; } } true } }
    };
}

impl_110!();