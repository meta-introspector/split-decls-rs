macro_rules! deps {
    () => {
        Pool!();
        Config!();
        Clear!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        unsafe impl < T , C > Send for Pool < T , C > where T : Send + Clear + Default , C : cfg :: Config , { }
    };
}

impl_13!();