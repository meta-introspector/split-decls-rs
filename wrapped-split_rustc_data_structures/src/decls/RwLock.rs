macro_rules! RwLock {
    () => {
        # [derive (Debug , Default)] pub struct RwLock < T > (parking_lot :: RwLock < T >) ;
    };
}

RwLock!();