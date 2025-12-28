macro_rules! IndexOccupiedEntry {
    () => {
        pub type IndexOccupiedEntry < 'a , K , V > = indexmap :: map :: OccupiedEntry < 'a , K , V > ;
    };
}

IndexOccupiedEntry!()