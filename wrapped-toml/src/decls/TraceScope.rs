macro_rules! deps {
    () => {
        DebugDepthGuard!();
        Style!();
    };
}

macro_rules! TraceScope {
    () => {
        deps!();
        pub (crate) struct TraceScope { text : String , style : anstyle :: Style , guard : DebugDepthGuard , }
    };
}

TraceScope!();