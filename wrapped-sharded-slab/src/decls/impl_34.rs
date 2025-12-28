macro_rules! deps {
    () => {
        Clear!();
        OwnedRef!();
        Config!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        unsafe impl < T , C > Send for OwnedRef < T , C > where T : Sync + Clear + Default , C : cfg :: Config , { }
    };
}

impl_34!()