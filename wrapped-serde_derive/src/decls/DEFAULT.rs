macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! DEFAULT {
    () => {
        deps!();
        pub const DEFAULT : Symbol = Symbol ("default") ;
    };
}

DEFAULT!()