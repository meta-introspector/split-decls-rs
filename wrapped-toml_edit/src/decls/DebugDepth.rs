macro_rules! DebugDepth {
    () => {
        pub (crate) struct DebugDepth (core :: sync :: atomic :: AtomicUsize) ;
    };
}

DebugDepth!();