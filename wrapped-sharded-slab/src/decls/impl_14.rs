macro_rules! deps {
    () => {
        Clear!();
        Pool!();
        Config!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        unsafe impl < T , C > Sync for Pool < T , C > where T : Sync + Clear + Default , C : cfg :: Config , { }
    };
}

impl_14!();