macro_rules! deps {
    () => {
        SessionGlobals!();
    };
}

macro_rules! with_session_globals {
    () => {
        deps!();
        # [inline] pub fn with_session_globals < R , F > (f : F) -> R where F : FnOnce (& SessionGlobals) -> R , { SESSION_GLOBALS . with (f) }
    };
}

with_session_globals!()