macro_rules! deps {
    () => {
        Config!();
        Slab!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        unsafe impl < T : Send , C : cfg :: Config > Send for Slab < T , C > { }
    };
}

impl_186!()