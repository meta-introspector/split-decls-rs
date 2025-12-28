macro_rules! deps {
    () => {
        Chunk!();
        Slice!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < A , Slice , const N : usize > PartialEq < Slice > for Chunk < A , N > where Slice : Borrow < [A] > , A : PartialEq , { fn eq (& self , other : & Slice) -> bool { self . as_slice () == other . borrow () } }
    };
}

impl_61!()