macro_rules! deps {
    () => {
        OwnedSlice!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl Deref for OwnedSlice { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { unsafe { & * self . bytes } } }
    };
}

impl_344!();