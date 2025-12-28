macro_rules! deps {
    () => {
        MmapMut!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl Deref for MmapMut { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { & self . 0 } }
    };
}

impl_304!()