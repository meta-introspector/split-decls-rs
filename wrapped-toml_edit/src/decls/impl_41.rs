macro_rules! deps {
    () => {
        Table!();
        DocumentMut!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl std :: ops :: Deref for DocumentMut { type Target = Table ; fn deref (& self) -> & Self :: Target { self . as_table () } }
    };
}

impl_41!();