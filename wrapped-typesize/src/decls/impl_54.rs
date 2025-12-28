macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl TypeSize for time :: OffsetDateTime { }
    };
}

impl_54!();