macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl From < & String > for ConstValue { # [inline] fn from (value : & String) -> Self { ConstValue :: String (value . clone ()) } }
    };
}

impl_106!();