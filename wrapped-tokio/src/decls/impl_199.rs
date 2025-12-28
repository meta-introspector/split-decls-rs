macro_rules! deps {
    () => {
        AtomicUsize!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl ops :: Deref for AtomicUsize { type Target = std :: sync :: atomic :: AtomicUsize ; fn deref (& self) -> & Self :: Target { unsafe { & * self . inner . get () } } }
    };
}

impl_199!();