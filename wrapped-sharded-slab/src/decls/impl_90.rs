macro_rules! deps {
    () => {
        Config!();
        Generation!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < C : cfg :: Config > Generation < C > { fn advance (self) -> Self { Self :: from_usize ((self . value + 1) % Self :: BITS) } }
    };
}

impl_90!()