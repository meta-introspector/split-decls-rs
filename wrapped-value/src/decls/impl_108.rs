macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < 'a > From < & 'a str > for ConstValue { # [inline] fn from (value : & 'a str) -> Self { ConstValue :: String (value . into ()) } }
    };
}

impl_108!()