macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl TypeSize for SystemTime { }
    };
}

impl_59!()