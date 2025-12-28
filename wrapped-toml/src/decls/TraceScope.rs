macro_rules! deps {
    () => {
        Style!();
        DebugDepthGuard!();
    };
}

macro_rules! TraceScope {
    () => {
        deps!();
        pub (crate) struct TraceScope { text : String , style : anstyle :: Style , guard : DebugDepthGuard , }
    };
}

TraceScope!()