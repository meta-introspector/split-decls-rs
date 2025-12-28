macro_rules! deps {
    () => {
        UnordCollection!();
        UnordSet!();
    };
}

macro_rules! impl_645 {
    () => {
        deps!();
        impl < V : Eq + Hash > UnordCollection for UnordSet < V > { }
    };
}

impl_645!();