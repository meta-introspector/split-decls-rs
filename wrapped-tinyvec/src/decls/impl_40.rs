macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < A : Array > FromIterator < A :: Item > for ArrayVec < A > { # [inline] fn from_iter < T : IntoIterator < Item = A :: Item > > (iter : T) -> Self { let mut av = Self :: default () ; for i in iter { av . push (i) } av } }
    };
}

impl_40!();