macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < 'b > From < & 'b str > for Key { fn from (s : & 'b str) -> Self { Self :: new (s) } }
    };
}

impl_141!();