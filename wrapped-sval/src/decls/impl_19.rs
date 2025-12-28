macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < 'a > Eq for Label < 'a > { }
    };
}

impl_19!()