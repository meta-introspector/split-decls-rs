// Generated macro for set_rustflags_w_warnings (function)
macro_rules! Depcrateset_rustflags_w_warnings {
() => {
// Module: crate
// Provides: {"set_rustflags_w_warnings"}
// Dependencies: {}
# [doc = " Sets `-Wwarnings` in `RUSTFLAGS`."] pub fn set_rustflags_w_warnings () { use parking_lot :: Mutex ; static ENV_MTX : Mutex < () > = parking_lot :: const_mutex (()) ; let guard = ENV_MTX . lock () ; let mut rustflags = env :: var_os ("RUSTFLAGS") . unwrap_or_default () ; rustflags . push (" -Wwarnings") ; env :: set_var ("RUSTFLAGS" , rustflags) ; std :: mem :: drop (guard) ; }
};
}
