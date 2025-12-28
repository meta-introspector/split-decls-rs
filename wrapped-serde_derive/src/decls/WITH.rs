macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! WITH {
    () => {
        deps!();
        pub const WITH : Symbol = Symbol ("with") ;
    };
}

WITH!()