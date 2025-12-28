macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! OTHER {
    () => {
        deps!();
        pub const OTHER : Symbol = Symbol ("other") ;
    };
}

OTHER!()