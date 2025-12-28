macro_rules! deps {
    () => {
        UnordSet!();
    };
}

macro_rules! impl_673 {
    () => {
        deps!();
        impl < V > ! IntoIterator for UnordSet < V > { }
    };
}

impl_673!();