macro_rules! deps {
    () => {
        Pair!();
        Punctuated!();
    };
}

macro_rules! impl_609 {
    () => {
        deps!();
        impl < T , P > FromIterator < Pair < T , P > > for Punctuated < T , P > { fn from_iter < I : IntoIterator < Item = Pair < T , P > > > (i : I) -> Self { let mut ret = Punctuated :: new () ; do_extend (& mut ret , i . into_iter ()) ; ret } }
    };
}

impl_609!();