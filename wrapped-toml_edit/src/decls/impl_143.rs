macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl From < String > for Key { fn from (s : String) -> Self { Self :: new (s) } }
    };
}

impl_143!()