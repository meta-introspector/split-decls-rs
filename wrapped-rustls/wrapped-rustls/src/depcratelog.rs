// Generated macro for log (module)
macro_rules! Depcratelog {
() => {
// Module: crate
// Provides: {"log"}
// Dependencies: {}
# [cfg (not (feature = "log"))] mod log { macro_rules ! trace (($ ($ tt : tt) *) => { crate :: log :: _used ! ($ ($ tt) *) }) ; macro_rules ! debug (($ ($ tt : tt) *) => { crate :: log :: _used ! ($ ($ tt) *) }) ; macro_rules ! error (($ ($ tt : tt) *) => { crate :: log :: _used ! ($ ($ tt) *) }) ; macro_rules ! _warn (($ ($ tt : tt) *) => { crate :: log :: _used ! ($ ($ tt) *) }) ; macro_rules ! _used (($ ($ tt : tt) *) => { { let _ = format_args ! ($ ($ tt) *) ; } }) ; pub (crate) use { _used , _warn as warn , debug , error , trace } ; }
};
}
