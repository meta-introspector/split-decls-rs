macro_rules! deps {
    () => {
        DebugDepth!();
    };
}

macro_rules! DEBUG_DEPTH {
    () => {
        deps!();
        static DEBUG_DEPTH : DebugDepth = DebugDepth (core :: sync :: atomic :: AtomicUsize :: new (0)) ;
    };
}

DEBUG_DEPTH!()