macro_rules! deps {
    () => {
        DefId!();
    };
}

macro_rules! impl_480 {
    () => {
        deps!();
        impl IndexedVal for DefId { fn to_val (index : usize) -> Self { DefId (index) } fn to_index (& self) -> usize { self . 0 } }
    };
}

impl_480!();