macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! DESERIALIZE_WITH {
    () => {
        deps!();
        pub const DESERIALIZE_WITH : Symbol = Symbol ("deserialize_with") ;
    };
}

DESERIALIZE_WITH!();