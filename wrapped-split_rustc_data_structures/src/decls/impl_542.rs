macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_542 {
    () => {
        deps!();
        impl < K , V , HCX > ! HashStable < HCX > for std :: collections :: HashMap < K , V > { }
    };
}

impl_542!()