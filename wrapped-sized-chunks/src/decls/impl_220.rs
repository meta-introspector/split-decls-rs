macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl < A , PrimSlice , const N : usize > PartialEq < PrimSlice > for RingBuffer < A , N > where PrimSlice : Borrow < [A] > , A : PartialEq , { # [inline] # [must_use] fn eq (& self , other : & PrimSlice) -> bool { let other = other . borrow () ; self . len () == other . len () && self . iter () . eq (other . iter ()) } }
    };
}

impl_220!()