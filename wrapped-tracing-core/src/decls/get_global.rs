macro_rules! deps {
    () => {
        Dispatch!();
    };
}

macro_rules! get_global {
    () => {
        deps!();
        # [inline] fn get_global () -> & 'static Dispatch { if GLOBAL_INIT . load (Ordering :: SeqCst) != INITIALIZED { return & NONE ; } unsafe { & * addr_of ! (GLOBAL_DISPATCH) } }
    };
}

get_global!();