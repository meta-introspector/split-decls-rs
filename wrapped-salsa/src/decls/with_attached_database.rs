macro_rules! deps {
    () => {
        Database!();
    };
}

macro_rules! with_attached_database {
    () => {
        deps!();
        # [doc = " Access the \"attached\" database. Returns `None` if no database is attached."] # [doc = " Databases are attached with `attach_database`."] # [inline] pub fn with_attached_database < R > (op : impl FnOnce (& dyn Database) -> R) -> Option < R > { ATTACHED . with (# [inline] | a | a . with (op) ,) }
    };
}

with_attached_database!()