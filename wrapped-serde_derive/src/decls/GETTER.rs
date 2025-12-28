macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! GETTER {
    () => {
        deps!();
        pub const GETTER : Symbol = Symbol ("getter") ;
    };
}

GETTER!();