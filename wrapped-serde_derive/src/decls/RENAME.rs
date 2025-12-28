macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! RENAME {
    () => {
        deps!();
        pub const RENAME : Symbol = Symbol ("rename") ;
    };
}

RENAME!();