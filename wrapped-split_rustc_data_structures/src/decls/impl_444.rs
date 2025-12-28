macro_rules! deps {
    () => {
        SsoHashMap!();
    };
}

macro_rules! impl_444 {
    () => {
        deps!();
        impl < K : Eq + Hash , V > FromIterator < (K , V) > for SsoHashMap < K , V > { fn from_iter < I : IntoIterator < Item = (K , V) > > (iter : I) -> SsoHashMap < K , V > { let mut map : SsoHashMap < K , V > = Default :: default () ; map . extend (iter) ; map } }
    };
}

impl_444!();