macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! VARIANT_IDENTIFIER {
    () => {
        deps!();
        pub const VARIANT_IDENTIFIER : Symbol = Symbol ("variant_identifier") ;
    };
}

VARIANT_IDENTIFIER!()