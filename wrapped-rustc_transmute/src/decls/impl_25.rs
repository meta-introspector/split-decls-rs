macro_rules! deps {
    () => {
        Byte!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl From < u8 > for Byte { # [inline] fn from (src : u8) -> Self { Self :: from_val (src) } }
    };
}

impl_25!()