macro_rules! deps {
    () => {
        Views!();
        Database!();
    };
}

macro_rules! views {
    () => {
        deps!();
        pub fn views < Db : ? Sized + Database > (db : & Db) -> & Views { db . zalsa () . views () }
    };
}

views!()