macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < A : Array > From < A > for ArrayVec < A > { # [inline (always)] # [doc = " The output has a length equal to the full array."] # [doc = ""] # [doc = " If you want to select a length, use"] # [doc = " [`from_array_len`](ArrayVec::from_array_len)"] fn from (data : A) -> Self { let len : u16 = data . as_slice () . len () . try_into () . expect ("ArrayVec::from> length must be in range 0..=u16::MAX") ; Self { len , data } } }
    };
}

impl_35!()