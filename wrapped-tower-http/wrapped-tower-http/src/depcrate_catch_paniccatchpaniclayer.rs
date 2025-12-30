// Generated macro for CatchPanicLayer (struct)
macro_rules! Depcrate_catch_panicCatchPanicLayer {
() => {
// Module: crate::catch_panic
// Provides: {"CatchPanicLayer"}
// Dependencies: {}
# [doc = " Layer that applies the [`CatchPanic`] middleware that catches panics and converts them into"] # [doc = " `500 Internal Server` responses."] # [doc = ""] # [doc = " See the [module docs](self) for an example."] # [derive (Debug , Clone , Copy , Default)] pub struct CatchPanicLayer < T > { panic_handler : T , }
};
}
