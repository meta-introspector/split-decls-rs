macro_rules! deps {
    () => {
        OwnedEntry!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        unsafe impl < T , C > Sync for OwnedEntry < T , C > where T : Sync , C : cfg :: Config , { }
    };
}

impl_32!()