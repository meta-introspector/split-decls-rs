macro_rules! deps {
    () => {
        Mmap!();
    };
}

macro_rules! impl_299 {
    () => {
        deps!();
        impl AsRef < [u8] > for Mmap { fn as_ref (& self) -> & [u8] { & self . 0 } }
    };
}

impl_299!();