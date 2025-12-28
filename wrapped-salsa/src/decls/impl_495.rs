macro_rules! deps {
    () => {
        Nonce!();
    };
}

macro_rules! impl_495 {
    () => {
        deps!();
        impl < T > Nonce < T > { pub (crate) fn into_u32 (self) -> NonZeroU32 { self . 0 } pub (crate) fn from_u32 (u32 : NonZeroU32) -> Self { Self (u32 , PhantomData) } }
    };
}

impl_495!();