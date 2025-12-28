macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! DENY_UNKNOWN_FIELDS {
    () => {
        deps!();
        pub const DENY_UNKNOWN_FIELDS : Symbol = Symbol ("deny_unknown_fields") ;
    };
}

DENY_UNKNOWN_FIELDS!();