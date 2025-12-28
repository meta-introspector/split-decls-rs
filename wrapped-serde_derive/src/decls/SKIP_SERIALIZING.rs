macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! SKIP_SERIALIZING {
    () => {
        deps!();
        pub const SKIP_SERIALIZING : Symbol = Symbol ("skip_serializing") ;
    };
}

SKIP_SERIALIZING!();