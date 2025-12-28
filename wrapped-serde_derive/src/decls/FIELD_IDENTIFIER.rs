macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! FIELD_IDENTIFIER {
    () => {
        deps!();
        pub const FIELD_IDENTIFIER : Symbol = Symbol ("field_identifier") ;
    };
}

FIELD_IDENTIFIER!()