macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! Symbol {
    () => {
        deps!();
        struct Symbol { name : Ident , value : Value , }
    };
}

Symbol!();