macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! SKIP_SERIALIZING_IF {
    () => {
        deps!();
        pub const SKIP_SERIALIZING_IF : Symbol = Symbol ("skip_serializing_if") ;
    };
}

SKIP_SERIALIZING_IF!()