macro_rules! deps {
    () => {
        Config!();
        Ref!();
        Clear!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < T , C > std :: ops :: Deref for Ref < '_ , T , C > where T : Clear + Default , C : cfg :: Config , { type Target = T ; fn deref (& self) -> & Self :: Target { self . value () } }
    };
}

impl_18!();