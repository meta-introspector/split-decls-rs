macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! EXPECTING {
    () => {
        deps!();
        pub const EXPECTING : Symbol = Symbol ("expecting") ;
    };
}

EXPECTING!()