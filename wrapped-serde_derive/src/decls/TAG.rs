macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! TAG {
    () => {
        deps!();
        pub const TAG : Symbol = Symbol ("tag") ;
    };
}

TAG!();