macro_rules! deps {
    () => {
        OpaqueId!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl OpaqueId { pub fn new () -> Self { use std :: sync :: atomic :: { AtomicU32 , Ordering } ; static OPAQUE_ID : AtomicU32 = AtomicU32 :: new (0) ; OpaqueId (OPAQUE_ID . fetch_add (1 , Ordering :: SeqCst)) } }
    };
}

impl_16!();