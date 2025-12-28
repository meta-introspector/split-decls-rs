macro_rules! deps {
    () => {
        OrderedF64!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl OrderedF64 { fn new (n : f64) -> Self { OrderedF64 (n) } }
    };
}

impl_127!();