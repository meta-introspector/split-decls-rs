macro_rules! deps {
    () => {
        Value!();
        Array!();
    };
}

macro_rules! impl_266 {
    () => {
        deps!();
        impl From < Array > for Value { fn from (array : Array) -> Self { Self :: Array (array) } }
    };
}

impl_266!();