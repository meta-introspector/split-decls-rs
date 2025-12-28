macro_rules! deps {
    () => {
        OwnedEntry!();
        Config!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        unsafe impl < T , C > Sync for OwnedEntry < T , C > where T : Sync , C : cfg :: Config , { }
    };
}

impl_200!();