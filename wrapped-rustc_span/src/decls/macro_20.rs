macro_rules! deps {
    () => {
        SessionGlobals!();
    };
}

macro_rules! macro_20 {
    () => {
        deps!();
        scoped_tls :: scoped_thread_local ! (static SESSION_GLOBALS : SessionGlobals) ;
    };
}

macro_20!()