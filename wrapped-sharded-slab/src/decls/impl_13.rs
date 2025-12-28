macro_rules! deps {
    () => {
        Clear!();
        Config!();
        Pool!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        unsafe impl < T , C > Send for Pool < T , C > where T : Send + Clear + Default , C : cfg :: Config , { }
    };
}

impl_13!()