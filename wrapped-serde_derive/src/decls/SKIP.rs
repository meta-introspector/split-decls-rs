macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! SKIP {
    () => {
        deps!();
        pub const SKIP : Symbol = Symbol ("skip") ;
    };
}

SKIP!()