macro_rules! deps {
    () => {
        Dispatch!();
    };
}

macro_rules! register_dispatch {
    () => {
        deps!();
        pub (crate) fn register_dispatch (dispatch : & Dispatch) { let dispatchers = DISPATCHERS . register_dispatch (dispatch) ; dispatch . subscriber () . on_register_dispatch (dispatch) ; CALLSITES . rebuild_interest (dispatchers) ; }
    };
}

register_dispatch!();