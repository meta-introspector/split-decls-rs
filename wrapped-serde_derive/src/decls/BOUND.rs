macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! BOUND {
    () => {
        deps!();
        pub const BOUND : Symbol = Symbol ("bound") ;
    };
}

BOUND!();