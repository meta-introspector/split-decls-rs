macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        unsafe impl < T : Send , C : cfg :: Config > Send for Slab < T , C > { }
    };
}

impl_18!()