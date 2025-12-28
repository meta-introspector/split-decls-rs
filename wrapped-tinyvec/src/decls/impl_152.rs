macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < A : Array > FromIterator < A :: Item > for TinyVec < A > { # [inline] fn from_iter < T : IntoIterator < Item = A :: Item > > (iter : T) -> Self { let mut av = Self :: default () ; av . extend (iter) ; av } }
    };
}

impl_152!()