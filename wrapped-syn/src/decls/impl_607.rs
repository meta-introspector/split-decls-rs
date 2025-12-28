macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! impl_607 {
    () => {
        deps!();
        impl < T , P > FromIterator < T > for Punctuated < T , P > where P : Default , { fn from_iter < I : IntoIterator < Item = T > > (i : I) -> Self { let mut ret = Punctuated :: new () ; ret . extend (i) ; ret } }
    };
}

impl_607!();