macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl AsRef < [u8] > for Uuid { # [inline] fn as_ref (& self) -> & [u8] { & self . 0 } }
    };
}

impl_144!()