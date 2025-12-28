macro_rules! deps {
    () => {
        UnordMap!();
    };
}

macro_rules! impl_659 {
    () => {
        deps!();
        impl < K : Hash + Eq , V > FromIterator < (K , V) > for UnordMap < K , V > { # [inline] fn from_iter < T : IntoIterator < Item = (K , V) > > (iter : T) -> Self { UnordMap { inner : FxHashMap :: from_iter (iter) } } }
    };
}

impl_659!();