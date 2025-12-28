macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'a > From < Cow < 'a , str > > for ConstValue { # [inline] fn from (f : Cow < 'a , str >) -> Self { ConstValue :: String (f . into_owned ()) } }
    };
}

impl_32!()