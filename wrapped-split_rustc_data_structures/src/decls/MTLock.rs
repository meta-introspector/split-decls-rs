macro_rules! MTLock {
    () => {
        # [derive (Debug , Default)] pub struct MTLock < T > (Lock < T >) ;
    };
}

MTLock!()