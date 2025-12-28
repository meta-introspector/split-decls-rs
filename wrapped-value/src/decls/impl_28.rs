macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl From < String > for ConstValue { # [inline] fn from (value : String) -> Self { ConstValue :: String (value) } }
    };
}

impl_28!()