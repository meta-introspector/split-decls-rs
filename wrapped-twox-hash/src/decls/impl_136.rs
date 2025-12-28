macro_rules! deps {
    () => {
        IntoU32!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl IntoU32 for u8 { fn into_u32 (self) -> u32 { self . into () } }
    };
}

impl_136!();