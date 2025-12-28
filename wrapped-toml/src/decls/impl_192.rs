macro_rules! deps {
    () => {
        DeValue!();
        DeArray!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl < 'i > FromIterator < Spanned < DeValue < 'i > > > for DeArray < 'i > { # [inline] # [track_caller] fn from_iter < I : IntoIterator < Item = Spanned < DeValue < 'i > > > > (iter : I) -> Self { Self { items : iter . into_iter () . collect () , array_of_tables : false , } } }
    };
}

impl_192!()