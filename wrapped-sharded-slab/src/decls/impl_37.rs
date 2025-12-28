macro_rules! deps {
    () => {
        OwnedRefMut!();
        Config!();
        Clear!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < T , C > std :: ops :: DerefMut for OwnedRefMut < T , C > where T : Clear + Default , C : cfg :: Config , { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { self . inner . value_mut () } } }
    };
}

impl_37!();