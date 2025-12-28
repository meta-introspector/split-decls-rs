macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < T : Into < ConstValue > > FromIterator < T > for ConstValue { fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { ConstValue :: List (iter . into_iter () . map (Into :: into) . collect ()) } }
    };
}

impl_33!()