macro_rules! deps {
    () => {
        ConstValue!();
        Value!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl From < ConstValue > for Value { fn from (value : ConstValue) -> Self { value . into_value () } }
    };
}

impl_123!()