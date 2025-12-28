macro_rules! EntryRef {
    () => {
        # [doc = " Generalisation over (&K, &V) and types like dashmap's `RefMulti`."] pub (crate) trait EntryRef < K , V > { fn get_ref (& self) -> (& K , & V) ; }
    };
}

EntryRef!();