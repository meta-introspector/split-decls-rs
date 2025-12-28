macro_rules! RwLock {
    () => {
        # [doc = " Adapter for `std::sync::RwLock` that removes the poisoning aspects"] # [doc = " from its api."] # [derive (Debug)] pub (crate) struct RwLock < T : ? Sized > (sync :: RwLock < T >) ;
    };
}

RwLock!()