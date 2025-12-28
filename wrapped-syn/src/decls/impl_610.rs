macro_rules! deps {
    () => {
        Pair!();
        Punctuated!();
    };
}

macro_rules! impl_610 {
    () => {
        deps!();
        impl < T , P > Extend < Pair < T , P > > for Punctuated < T , P > where P : Default , { fn extend < I : IntoIterator < Item = Pair < T , P > > > (& mut self , i : I) { if ! self . empty_or_trailing () { self . push_punct (P :: default ()) ; } do_extend (self , i . into_iter ()) ; } }
    };
}

impl_610!()