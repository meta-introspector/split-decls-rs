macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl From < & String > for ConstValue { # [inline] fn from (value : & String) -> Self { ConstValue :: String (value . clone ()) } }
    };
}

impl_29!()