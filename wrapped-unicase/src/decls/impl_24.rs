macro_rules! deps {
    () => {
        Unicode!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < S : AsRef < str > > Eq for Unicode < S > { }
    };
}

impl_24!()