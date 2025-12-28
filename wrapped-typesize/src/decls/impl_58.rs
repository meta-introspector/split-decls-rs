macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl TypeSize for Instant { }
    };
}

impl_58!();