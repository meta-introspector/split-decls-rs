macro_rules! deps {
    () => {
        Formatted!();
        Value!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl From < Datetime > for Value { fn from (d : Datetime) -> Self { Self :: Datetime (Formatted :: new (d)) } }
    };
}

impl_263!();