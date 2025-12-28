macro_rules! deps {
    () => {
        Item!();
        Value!();
        Array!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl < V : Into < Self > > FromIterator < V > for Value { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = V > , { let array : Array = iter . into_iter () . collect () ; Self :: Array (array) } }
    };
}

impl_268!()