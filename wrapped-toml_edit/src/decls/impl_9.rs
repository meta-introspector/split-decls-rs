macro_rules! deps {
    () => {
        Array!();
        Value!();
        Item!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < V : Into < Value > > FromIterator < V > for Array { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = V > , { let v = iter . into_iter () . map (| a | Item :: Value (a . into ())) ; Self { values : v . collect () , .. Default :: default () } } }
    };
}

impl_9!();