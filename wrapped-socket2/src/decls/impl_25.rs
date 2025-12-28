macro_rules! deps {
    () => {
        MaybeUninitSlice!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'a > Deref for MaybeUninitSlice < 'a > { type Target = [MaybeUninit < u8 >] ; fn deref (& self) -> & [MaybeUninit < u8 >] { self . 0 . as_slice () } }
    };
}

impl_25!()