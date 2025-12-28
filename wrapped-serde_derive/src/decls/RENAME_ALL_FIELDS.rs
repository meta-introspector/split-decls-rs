macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! RENAME_ALL_FIELDS {
    () => {
        deps!();
        pub const RENAME_ALL_FIELDS : Symbol = Symbol ("rename_all_fields") ;
    };
}

RENAME_ALL_FIELDS!();