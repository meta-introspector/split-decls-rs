macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! IndexEntry {
    () => {
        deps!();
        pub type IndexEntry < 'a , K , V > = indexmap :: map :: Entry < 'a , K , V > ;
    };
}

IndexEntry!();