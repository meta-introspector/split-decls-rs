macro_rules! deps {
    () => {
        Key!();
        InlineTable!();
        Item!();
        Value!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < K : Into < Key > , V : Into < Value > > FromIterator < (K , V) > for InlineTable { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = (K , V) > , { let mut table = Self :: new () ; table . extend (iter) ; table } }
    };
}

impl_95!()