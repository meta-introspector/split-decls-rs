macro_rules! deps {
    () => {
        Value!();
        Formatted!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl From < Datetime > for Value { fn from (d : Datetime) -> Self { Self :: Datetime (Formatted :: new (d)) } }
    };
}

impl_263!()