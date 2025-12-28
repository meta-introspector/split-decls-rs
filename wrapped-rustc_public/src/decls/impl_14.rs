macro_rules! impl_14 {
    () => {
        impl IndexedVal for DefId { fn to_val (index : usize) -> Self { DefId (index) } fn to_index (& self) -> usize { self . 0 } }
    };
}

impl_14!()