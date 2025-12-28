macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T : Into < ConstValue > > From < Vec < T > > for ConstValue { fn from (f : Vec < T >) -> Self { ConstValue :: List (f . into_iter () . map (Into :: into) . collect ()) } }
    };
}

impl_35!()