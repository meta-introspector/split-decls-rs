macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! SERDE {
    () => {
        deps!();
        pub const SERDE : Symbol = Symbol ("serde") ;
    };
}

SERDE!();