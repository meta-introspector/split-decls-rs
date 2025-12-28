macro_rules! deps {
    () => {
        QueryResult!();
    };
}

macro_rules! QueryState {
    () => {
        deps!();
        pub struct QueryState < K , I > { active : Sharded < hashbrown :: HashTable < (K , QueryResult < I >) > > , }
    };
}

QueryState!()