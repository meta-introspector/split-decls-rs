macro_rules! deps {
    () => {
        UniqueReentrantMutex!();
    };
}

macro_rules! check_new_key {
    () => {
        deps!();
        pub (crate) fn check_new_key (name : & str) { if global_locks () . contains (name) { return ; } ; let entry = global_locks () . entry (name . to_owned ()) ; match entry { Entry :: Occupied (o) => o , Entry :: Vacant (v) => v . insert_entry (UniqueReentrantMutex :: new_mutex (name)) , } ; }
    };
}

check_new_key!()