macro_rules! deps {
    () => {
        Config!();
        Entry!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        unsafe impl < T , C > Send for Entry < '_ , T , C > where T : Sync , C : cfg :: Config , { }
    };
}

impl_193!()