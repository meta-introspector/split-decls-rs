macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! SKIP_DESERIALIZING {
    () => {
        deps!();
        pub const SKIP_DESERIALIZING : Symbol = Symbol ("skip_deserializing") ;
    };
}

SKIP_DESERIALIZING!();