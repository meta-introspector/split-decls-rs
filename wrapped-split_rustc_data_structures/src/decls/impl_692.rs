macro_rules! deps {
    () => {
        AtomicRef!();
    };
}

macro_rules! impl_692 {
    () => {
        deps!();
        impl < T : 'static > std :: ops :: Deref for AtomicRef < T > { type Target = T ; fn deref (& self) -> & Self :: Target { unsafe { & * self . 0 . load (Ordering :: SeqCst) } } }
    };
}

impl_692!()