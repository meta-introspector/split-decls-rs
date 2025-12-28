macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < 'a > From < & 'a str > for Value { # [inline] fn from (val : & 'a str) -> Self { Self :: String (val . to_owned ()) } }
    };
}

impl_57!();