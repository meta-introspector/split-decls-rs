macro_rules! deps {
    () => {
        UnordMap!();
    };
}

macro_rules! impl_674 {
    () => {
        deps!();
        impl < K , V > ! IntoIterator for UnordMap < K , V > { }
    };
}

impl_674!()