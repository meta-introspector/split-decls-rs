macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! TRY_FROM {
    () => {
        deps!();
        pub const TRY_FROM : Symbol = Symbol ("try_from") ;
    };
}

TRY_FROM!()