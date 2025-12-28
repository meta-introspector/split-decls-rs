macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        unsafe impl < 'computed > Sync for Label < 'computed > { }
    };
}

impl_15!()