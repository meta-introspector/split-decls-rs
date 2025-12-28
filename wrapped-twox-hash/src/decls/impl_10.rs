macro_rules! deps {
    () => {
        IntoU64!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl IntoU64 for u8 { fn into_u64 (self) -> u64 { self . into () } }
    };
}

impl_10!()