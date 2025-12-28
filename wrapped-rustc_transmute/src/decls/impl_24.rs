macro_rules! deps {
    () => {
        Byte!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl From < RangeInclusive < u8 > > for Byte { fn from (src : RangeInclusive < u8 >) -> Self { Self :: new (src) } }
    };
}

impl_24!()