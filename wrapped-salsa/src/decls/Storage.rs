macro_rules! deps {
    () => {
        ZalsaLocal!();
        Database!();
        StorageHandle!();
    };
}

macro_rules! Storage {
    () => {
        deps!();
        # [doc = " Concrete implementation of the [`Database`] trait with local state that can be used to drive computations."] pub struct Storage < Db > { handle : StorageHandle < Db > , # [doc = " Per-thread state"] zalsa_local : zalsa_local :: ZalsaLocal , }
    };
}

Storage!()