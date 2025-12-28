macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! NON_EXHAUSTIVE {
    () => {
        deps!();
        pub const NON_EXHAUSTIVE : Symbol = Symbol ("non_exhaustive") ;
    };
}

NON_EXHAUSTIVE!()