macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! RENAME_ALL {
    () => {
        deps!();
        pub const RENAME_ALL : Symbol = Symbol ("rename_all") ;
    };
}

RENAME_ALL!();