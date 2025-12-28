macro_rules! deps {
    () => {
        Args!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl std :: ops :: Deref for Args { type Target = [String] ; fn deref (& self) -> & Self :: Target { self . as_slice () } }
    };
}

impl_18!();