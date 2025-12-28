macro_rules! deps {
    () => {
        Slice!();
        InlineArray!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < A , T , Slice > PartialEq < Slice > for InlineArray < A , T > where Slice : Borrow < [A] > , A : PartialEq , { fn eq (& self , other : & Slice) -> bool { self . deref () == other . borrow () } }
    };
}

impl_23!()