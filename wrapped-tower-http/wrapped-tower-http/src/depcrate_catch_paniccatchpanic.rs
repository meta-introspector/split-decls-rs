// Generated macro for CatchPanic (struct)
macro_rules! Depcrate_catch_panicCatchPanic {
() => {
// Module: crate::catch_panic
// Provides: {"CatchPanic"}
// Dependencies: {}
# [doc = " Middleware that catches panics and converts them into `500 Internal Server` responses."] # [doc = ""] # [doc = " See the [module docs](self) for an example."] # [derive (Debug , Clone , Copy)] pub struct CatchPanic < S , T > { inner : S , panic_handler : T , }
};
}
