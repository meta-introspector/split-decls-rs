macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        unsafe impl < T , C > Send for Entry < '_ , T , C > where T : Sync , C : cfg :: Config , { }
    };
}

impl_25!()