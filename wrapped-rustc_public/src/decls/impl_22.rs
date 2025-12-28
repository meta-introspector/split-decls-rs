macro_rules! deps {
    () => {
        Layout!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl crate :: IndexedVal for Layout { fn to_val (index : usize) -> Self { Layout (index) } fn to_index (& self) -> usize { self . 0 } }
    };
}

impl_22!()