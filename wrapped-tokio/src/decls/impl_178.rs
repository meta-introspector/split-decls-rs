macro_rules! deps {
    () => {
        AtomicU16!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl Deref for AtomicU16 { type Target = std :: sync :: atomic :: AtomicU16 ; fn deref (& self) -> & Self :: Target { unsafe { & * self . inner . get () } } }
    };
}

impl_178!()