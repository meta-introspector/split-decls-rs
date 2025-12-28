macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl From < () > for ConstValue { fn from (() : ()) -> Self { ConstValue :: Null } }
    };
}

impl_22!()