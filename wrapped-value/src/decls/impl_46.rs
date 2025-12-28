macro_rules! deps {
    () => {
        Value!();
        ConstValue!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl From < ConstValue > for Value { fn from (value : ConstValue) -> Self { value . into_value () } }
    };
}

impl_46!()