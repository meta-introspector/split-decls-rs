macro_rules! deps {
    () => {
        OrderedF64!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl Eq for OrderedF64 { }
    };
}

impl_128!()