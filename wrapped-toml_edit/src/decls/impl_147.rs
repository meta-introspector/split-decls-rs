macro_rules! deps {
    () => {
        KeyMut!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl std :: ops :: Deref for KeyMut < '_ > { type Target = str ; fn deref (& self) -> & Self :: Target { self . get () } }
    };
}

impl_147!();