macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! DESERIALIZE {
    () => {
        deps!();
        pub const DESERIALIZE : Symbol = Symbol ("deserialize") ;
    };
}

DESERIALIZE!()