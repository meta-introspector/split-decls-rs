macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl Eq for Value { }
    };
}

impl_51!()