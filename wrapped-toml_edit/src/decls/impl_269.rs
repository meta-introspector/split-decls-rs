macro_rules! deps {
    () => {
        Value!();
        Item!();
        InlineTable!();
        Key!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl < K : Into < Key > , V : Into < Self > > FromIterator < (K , V) > for Value { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = (K , V) > , { let table : InlineTable = iter . into_iter () . collect () ; Self :: InlineTable (table) } }
    };
}

impl_269!();