macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! SERIALIZE_WITH {
    () => {
        deps!();
        pub const SERIALIZE_WITH : Symbol = Symbol ("serialize_with") ;
    };
}

SERIALIZE_WITH!();