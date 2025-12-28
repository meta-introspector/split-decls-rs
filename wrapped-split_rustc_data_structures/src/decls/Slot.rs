macro_rules! Slot {
    () => {
        struct Slot < V > { value : V , index_and_lock : AtomicU32 , }
    };
}

Slot!();