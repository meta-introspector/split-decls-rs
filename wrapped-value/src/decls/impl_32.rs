macro_rules! deps {
    () => {
        Extensions!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl DerefMut for Extensions { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_32!()