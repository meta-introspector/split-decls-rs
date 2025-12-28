macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! REMOTE {
    () => {
        deps!();
        pub const REMOTE : Symbol = Symbol ("remote") ;
    };
}

REMOTE!()