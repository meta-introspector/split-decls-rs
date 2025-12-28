macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! BORROW {
    () => {
        deps!();
        pub const BORROW : Symbol = Symbol ("borrow") ;
    };
}

BORROW!();