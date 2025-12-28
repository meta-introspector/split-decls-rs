macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_541 {
    () => {
        deps!();
        impl < V , HCX > ! HashStable < HCX > for std :: collections :: HashSet < V > { }
    };
}

impl_541!();