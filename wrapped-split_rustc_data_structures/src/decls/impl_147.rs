macro_rules! deps {
    () => {
        ReversedGraph!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < G > ReversedGraph < G > { pub fn new (inner : G) -> Self { Self { inner } } }
    };
}

impl_147!();