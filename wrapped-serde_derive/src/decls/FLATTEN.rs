macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! FLATTEN {
    () => {
        deps!();
        pub const FLATTEN : Symbol = Symbol ("flatten") ;
    };
}

FLATTEN!()