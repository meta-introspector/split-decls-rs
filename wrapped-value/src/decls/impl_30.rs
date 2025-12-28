macro_rules! deps {
    () => {
        Name!();
        ConstValue!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl From < Name > for ConstValue { # [inline] fn from (value : Name) -> Self { ConstValue :: Enum (value) } }
    };
}

impl_30!()