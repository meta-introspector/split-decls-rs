macro_rules! deps {
    () => {
        AtomicUsize!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl ops :: DerefMut for AtomicUsize { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { & mut * self . inner . get () } } }
    };
}

impl_200!();