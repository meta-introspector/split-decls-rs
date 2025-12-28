macro_rules! deps {
    () => {
        UnordSet!();
    };
}

macro_rules! impl_651 {
    () => {
        deps!();
        impl < V : Hash + Eq > FromIterator < V > for UnordSet < V > { # [inline] fn from_iter < T : IntoIterator < Item = V > > (iter : T) -> Self { UnordSet { inner : FxHashSet :: from_iter (iter) } } }
    };
}

impl_651!()