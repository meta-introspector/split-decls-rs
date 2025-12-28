macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl From < bool > for ConstValue { # [inline] fn from (value : bool) -> Self { ConstValue :: Boolean (value) } }
    };
}

impl_104!()