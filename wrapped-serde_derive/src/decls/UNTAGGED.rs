macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! UNTAGGED {
    () => {
        deps!();
        pub const UNTAGGED : Symbol = Symbol ("untagged") ;
    };
}

UNTAGGED!();