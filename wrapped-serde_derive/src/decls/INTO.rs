macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! INTO {
    () => {
        deps!();
        pub const INTO : Symbol = Symbol ("into") ;
    };
}

INTO!();