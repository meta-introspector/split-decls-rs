macro_rules! deps {
    () => {
        Database!();
        Revision!();
    };
}

macro_rules! current_revision {
    () => {
        deps!();
        pub fn current_revision < Db : ? Sized + Database > (db : & Db) -> Revision { db . zalsa () . current_revision () }
    };
}

current_revision!();