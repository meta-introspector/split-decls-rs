macro_rules! deps {
    () => {
        NoDrop!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < T : ? Sized > Deref for NoDrop < T > { type Target = T ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_161!()