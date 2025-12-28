macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < I > Unpin for Iter < I > { }
    };
}

impl_51!()