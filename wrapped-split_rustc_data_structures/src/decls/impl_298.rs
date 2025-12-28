macro_rules! deps {
    () => {
        Mmap!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl Deref for Mmap { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { & self . 0 } }
    };
}

impl_298!();