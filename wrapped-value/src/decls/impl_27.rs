macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl From < bool > for ConstValue { # [inline] fn from (value : bool) -> Self { ConstValue :: Boolean (value) } }
    };
}

impl_27!()