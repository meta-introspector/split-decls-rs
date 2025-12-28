macro_rules! deps {
    () => {
        MaybeUninitSlice!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < 'a > DerefMut for MaybeUninitSlice < 'a > { fn deref_mut (& mut self) -> & mut [MaybeUninit < u8 >] { self . 0 . as_mut_slice () } }
    };
}

impl_68!();