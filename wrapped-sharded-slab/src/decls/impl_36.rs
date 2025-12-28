macro_rules! deps {
    () => {
        Clear!();
        OwnedRefMut!();
        Config!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < T , C > std :: ops :: Deref for OwnedRefMut < T , C > where T : Clear + Default , C : cfg :: Config , { type Target = T ; fn deref (& self) -> & Self :: Target { self . value () } }
    };
}

impl_36!();