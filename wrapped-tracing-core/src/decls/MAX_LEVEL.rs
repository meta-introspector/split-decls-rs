macro_rules! deps {
    () => {
        LevelFilter!();
    };
}

macro_rules! MAX_LEVEL {
    () => {
        deps!();
        static MAX_LEVEL : AtomicUsize = AtomicUsize :: new (LevelFilter :: OFF_USIZE) ;
    };
}

MAX_LEVEL!();