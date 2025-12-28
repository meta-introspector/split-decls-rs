macro_rules! deps {
    () => {
        OwnedEntry!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T , C > std :: ops :: Deref for OwnedEntry < T , C > where C : cfg :: Config , { type Target = T ; fn deref (& self) -> & Self :: Target { self . value () } }
    };
}

impl_28!()