macro_rules! deps {
    () => {
        SortedMap!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        impl < K : Ord , V > IntoIterator for SortedMap < K , V > { type Item = (K , V) ; type IntoIter = std :: vec :: IntoIter < (K , V) > ; fn into_iter (self) -> Self :: IntoIter { self . data . into_iter () } }
    };
}

impl_431!()