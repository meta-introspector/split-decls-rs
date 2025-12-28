macro_rules! deps {
    () => {
        SessionGlobals!();
    };
}

macro_rules! macro_244 {
    () => {
        deps!();
        scoped_tls :: scoped_thread_local ! (static SESSION_GLOBALS : SessionGlobals) ;
    };
}

macro_244!();