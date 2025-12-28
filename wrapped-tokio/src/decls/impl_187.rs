macro_rules! deps {
    () => {
        AtomicU32!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl Deref for AtomicU32 { type Target = std :: sync :: atomic :: AtomicU32 ; fn deref (& self) -> & Self :: Target { unsafe { & * self . inner . get () } } }
    };
}

impl_187!()