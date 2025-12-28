macro_rules! deps {
    () => {
        DatabaseKeyIndex!();
        SalsaStructInDb!();
        Id!();
        Zalsa!();
    };
}

macro_rules! TrackedStructInDb {
    () => {
        deps!();
        pub trait TrackedStructInDb : SalsaStructInDb { # [doc = " Converts the identifier for this tracked struct into a `DatabaseKeyIndex`."] fn database_key_index (zalsa : & Zalsa , id : Id) -> DatabaseKeyIndex ; }
    };
}

TrackedStructInDb!();