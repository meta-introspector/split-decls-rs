macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! ALIAS {
    () => {
        deps!();
        pub const ALIAS : Symbol = Symbol ("alias") ;
    };
}

ALIAS!()