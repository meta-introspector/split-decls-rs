macro_rules! deps {
    () => {
        Config!();
        OwnedRef!();
        Clear!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        unsafe impl < T , C > Sync for OwnedRef < T , C > where T : Sync + Clear + Default , C : cfg :: Config , { }
    };
}

impl_33!()