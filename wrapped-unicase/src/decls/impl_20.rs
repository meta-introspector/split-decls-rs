macro_rules! deps {
    () => {
        UniCase!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < S : AsRef < str > > Eq for UniCase < S > { }
    };
}

impl_20!()