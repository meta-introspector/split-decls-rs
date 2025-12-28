macro_rules! deps {
    () => {
        Config!();
        Slab!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        unsafe impl < T : Sync , C : cfg :: Config > Sync for Slab < T , C > { }
    };
}

impl_187!();