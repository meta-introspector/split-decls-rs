macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! FROM {
    () => {
        deps!();
        pub const FROM : Symbol = Symbol ("from") ;
    };
}

FROM!();