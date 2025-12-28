macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! StdEntry {
    () => {
        deps!();
        pub type StdEntry < 'a , K , V > = std :: collections :: hash_map :: Entry < 'a , K , V > ;
    };
}

StdEntry!();