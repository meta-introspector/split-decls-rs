macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < 'b > From < & 'b String > for Key { fn from (s : & 'b String) -> Self { Self :: new (s) } }
    };
}

impl_142!();