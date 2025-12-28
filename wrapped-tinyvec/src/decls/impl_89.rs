macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < 's , T , A > From < & 's mut A > for SliceVec < 's , T > where A : AsMut < [T] > , { # [doc = " Calls `AsRef::as_mut` then uses the full slice as the initial length."] # [doc = " ## Example"] # [doc = " ```rust"] # [doc = " # use tinyvec::*;"] # [doc = " let mut arr = [0, 0];"] # [doc = " let mut sv = SliceVec::from(&mut arr);"] # [doc = " ```"] # [inline] fn from (a : & 's mut A) -> Self { let data = a . as_mut () ; let len = data . len () ; Self { data , len } } }
    };
}

impl_89!()