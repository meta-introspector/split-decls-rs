macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl From < String > for ConstValue { # [inline] fn from (value : String) -> Self { ConstValue :: String (value) } }
    };
}

impl_105!()