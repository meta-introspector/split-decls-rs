macro_rules! deps {
    () => {
        NoDrop!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < T : ? Sized > DerefMut for NoDrop < T > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_162!();