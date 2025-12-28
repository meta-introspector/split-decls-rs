macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! REPR {
    () => {
        deps!();
        pub const REPR : Symbol = Symbol ("repr") ;
    };
}

REPR!()