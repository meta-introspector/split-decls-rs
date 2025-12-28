macro_rules! deps {
    () => {
        OwnedEntry!();
        Config!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        unsafe impl < T , C > Send for OwnedEntry < T , C > where T : Sync , C : cfg :: Config , { }
    };
}

impl_201!()