macro_rules! deps {
    () => {
        IntoU64!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl IntoU64 for u32 { fn into_u64 (self) -> u64 { self . into () } }
    };
}

impl_11!()