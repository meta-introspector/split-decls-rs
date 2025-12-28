macro_rules! deps {
    () => {
        ShardedHashMap!();
        IntoPointer!();
    };
}

macro_rules! impl_397 {
    () => {
        deps!();
        impl < K : Eq + Hash + Copy + IntoPointer > ShardedHashMap < K , () > { pub fn contains_pointer_to < T : Hash + IntoPointer > (& self , value : & T) -> bool { let hash = make_hash (& value) ; let shard = self . lock_shard_by_hash (hash) ; let value = value . into_pointer () ; shard . find (hash , | (k , ()) | k . into_pointer () == value) . is_some () } }
    };
}

impl_397!()