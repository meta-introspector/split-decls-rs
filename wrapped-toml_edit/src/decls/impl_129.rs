macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl std :: ops :: Deref for Key { type Target = str ; fn deref (& self) -> & Self :: Target { self . get () } }
    };
}

impl_129!()