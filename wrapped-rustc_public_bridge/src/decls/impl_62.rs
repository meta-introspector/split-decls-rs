macro_rules! deps {
    () => {
        IndexedVal!();
        IndexMap!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < K : PartialEq + Hash + Eq , V : Copy + Debug + PartialEq + IndexedVal > IndexMap < K , V > { pub fn create_or_fetch (& mut self , key : K) -> V { let len = self . index_map . len () ; let v = self . index_map . entry (key) . or_insert (V :: to_val (len)) ; * v } }
    };
}

impl_62!();