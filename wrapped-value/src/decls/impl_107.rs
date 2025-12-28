macro_rules! deps {
    () => {
        Name!();
        ConstValue!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl From < Name > for ConstValue { # [inline] fn from (value : Name) -> Self { ConstValue :: Enum (value) } }
    };
}

impl_107!()