macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! CONTENT {
    () => {
        deps!();
        pub const CONTENT : Symbol = Symbol ("content") ;
    };
}

CONTENT!()