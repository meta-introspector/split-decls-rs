macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! impl_608 {
    () => {
        deps!();
        impl < T , P > Extend < T > for Punctuated < T , P > where P : Default , { fn extend < I : IntoIterator < Item = T > > (& mut self , i : I) { for value in i { self . push (value) ; } } }
    };
}

impl_608!()