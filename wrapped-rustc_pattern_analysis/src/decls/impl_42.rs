macro_rules! deps {
    () => {
        PatId!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl PatId { fn new () -> Self { use std :: sync :: atomic :: { AtomicU32 , Ordering } ; static PAT_ID : AtomicU32 = AtomicU32 :: new (0) ; PatId (PAT_ID . fetch_add (1 , Ordering :: SeqCst)) } }
    };
}

impl_42!()