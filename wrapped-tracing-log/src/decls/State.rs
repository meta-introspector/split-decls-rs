macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! State {
    () => {
        deps!();
        struct State { min_verbosity : Level , epoch : usize , cache : LruCache < Key , u64 , ahash :: RandomState > , }
    };
}

State!();