macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T : Eq > Eq for Spanned < T > { }
    };
}

impl_15!()