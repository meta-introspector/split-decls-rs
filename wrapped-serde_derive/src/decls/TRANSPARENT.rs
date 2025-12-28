macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! TRANSPARENT {
    () => {
        deps!();
        pub const TRANSPARENT : Symbol = Symbol ("transparent") ;
    };
}

TRANSPARENT!();