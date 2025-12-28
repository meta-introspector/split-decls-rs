macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        unsafe impl < 'computed > Send for Label < 'computed > { }
    };
}

impl_14!()