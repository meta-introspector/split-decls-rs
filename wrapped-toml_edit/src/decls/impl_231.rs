macro_rules! deps {
    () => {
        Key!();
        Item!();
        Table!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl < K : Into < Key > , V : Into < Item > > FromIterator < (K , V) > for Table { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = (K , V) > , { let mut table = Self :: new () ; table . extend (iter) ; table } }
    };
}

impl_231!()