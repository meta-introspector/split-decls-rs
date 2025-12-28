macro_rules! deps {
    () => {
        SortedMap!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        impl < K : Ord , V > FromIterator < (K , V) > for SortedMap < K , V > { fn from_iter < T : IntoIterator < Item = (K , V) > > (iter : T) -> Self { let mut data : Vec < (K , V) > = iter . into_iter () . collect () ; data . sort_unstable_by (| (k1 , _) , (k2 , _) | k1 . cmp (k2)) ; data . dedup_by (| (k1 , _) , (k2 , _) | k1 == k2) ; SortedMap { data } } }
    };
}

impl_434!();