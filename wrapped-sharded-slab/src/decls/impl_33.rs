macro_rules! deps {
    () => {
        OwnedEntry!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        unsafe impl < T , C > Send for OwnedEntry < T , C > where T : Sync , C : cfg :: Config , { }
    };
}

impl_33!()