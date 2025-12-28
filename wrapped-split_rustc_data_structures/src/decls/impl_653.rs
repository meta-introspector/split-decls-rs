macro_rules! deps {
    () => {
        UnordSet!();
        UnordItems!();
    };
}

macro_rules! impl_653 {
    () => {
        deps!();
        impl < V : Hash + Eq , I : Iterator < Item = V > > From < UnordItems < V , I > > for UnordSet < V > { fn from (value : UnordItems < V , I >) -> Self { UnordSet { inner : FxHashSet :: from_iter (value . 0) } } }
    };
}

impl_653!();