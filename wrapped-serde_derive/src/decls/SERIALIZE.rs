macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! SERIALIZE {
    () => {
        deps!();
        pub const SERIALIZE : Symbol = Symbol ("serialize") ;
    };
}

SERIALIZE!();