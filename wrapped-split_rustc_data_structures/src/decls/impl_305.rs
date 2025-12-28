macro_rules! deps {
    () => {
        MmapMut!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl DerefMut for MmapMut { # [inline] fn deref_mut (& mut self) -> & mut [u8] { & mut self . 0 } }
    };
}

impl_305!()