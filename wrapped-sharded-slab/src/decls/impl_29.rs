macro_rules! deps {
    () => {
        OwnedRef!();
        Clear!();
        Config!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < T , C > std :: ops :: Deref for OwnedRef < T , C > where T : Clear + Default , C : cfg :: Config , { type Target = T ; fn deref (& self) -> & Self :: Target { self . value () } }
    };
}

impl_29!()