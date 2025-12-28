macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! CRATE {
    () => {
        deps!();
        pub const CRATE : Symbol = Symbol ("crate") ;
    };
}

CRATE!();