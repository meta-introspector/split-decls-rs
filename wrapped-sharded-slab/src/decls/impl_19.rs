macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        unsafe impl < T : Sync , C : cfg :: Config > Sync for Slab < T , C > { }
    };
}

impl_19!()