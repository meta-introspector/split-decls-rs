macro_rules! deps {
    () => {
        IndexMap!();
        IndexedVal!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < K : PartialEq + Hash + Eq , V : Copy + Debug + PartialEq + IndexedVal > Index < V > for IndexMap < K , V > { type Output = K ; fn index (& self , index : V) -> & Self :: Output { let (k , v) = self . index_map . get_index (index . to_index ()) . unwrap () ; assert_eq ! (* v , index , "Provided value doesn't match with indexed value") ; k } }
    };
}

impl_63!()