macro_rules! deps {
    () => {
        Map!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < K : Ord + Hash , V > FromIterator < (K , V) > for Map < K , V > { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = (K , V) > , { Self { map : FromIterator :: from_iter (iter) , dotted : false , implicit : false , inline : false , } } }
    };
}

impl_15!();