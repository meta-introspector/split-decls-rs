macro_rules! deps {
    () => {
        MaybeUninitSlice!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < 'a > MaybeUninitSlice < 'a > { # [doc = " Creates a new `MaybeUninitSlice` wrapping a byte slice."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics on Windows if the slice is larger than 4GB."] pub fn new (buf : & 'a mut [MaybeUninit < u8 >]) -> MaybeUninitSlice < 'a > { MaybeUninitSlice (sys :: MaybeUninitSlice :: new (buf)) } }
    };
}

impl_24!()