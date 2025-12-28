macro_rules! deps {
    () => {
        Clear!();
        RefMut!();
        Config!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < T , C : cfg :: Config > std :: ops :: Deref for RefMut < '_ , T , C > where T : Clear + Default , C : cfg :: Config , { type Target = T ; fn deref (& self) -> & Self :: Target { self . value () } }
    };
}

impl_23!()