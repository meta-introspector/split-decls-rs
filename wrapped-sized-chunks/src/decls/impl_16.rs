macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < A , T > Clone for InlineArray < A , T > where A : Clone , { fn clone (& self) -> Self { let mut copy = Self :: new () ; for i in 0 .. self . len () { unsafe { copy . write_at (i , self . get_unchecked (i) . clone ()) ; } } unsafe { * copy . len_mut () = self . len () ; } copy } }
    };
}

impl_16!();