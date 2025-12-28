macro_rules! deps {
    () => {
        UnordSet!();
    };
}

macro_rules! impl_652 {
    () => {
        deps!();
        impl < V : Hash + Eq > From < FxHashSet < V > > for UnordSet < V > { fn from (value : FxHashSet < V >) -> Self { UnordSet { inner : value } } }
    };
}

impl_652!();