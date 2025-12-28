macro_rules! deps {
    () => {
        Value!();
        Formatted!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        impl From < String > for Value { fn from (s : String) -> Self { Self :: String (Formatted :: new (s)) } }
    };
}

impl_259!()