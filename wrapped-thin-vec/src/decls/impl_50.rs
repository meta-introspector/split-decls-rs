macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < T > FromIterator < T > for ThinVec < T > { # [inline] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> ThinVec < T > { let mut vec = ThinVec :: new () ; vec . extend (iter) ; vec } }
    };
}

impl_50!()