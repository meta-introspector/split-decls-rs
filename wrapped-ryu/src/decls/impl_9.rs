macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Float for f32 { }
    };
}

impl_9!()