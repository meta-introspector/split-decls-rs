// Generated macro for panic_any (function)
macro_rules! Depcrate_panicpanic_any {
() => {
// Module: crate::panic
// Provides: {"panic_any"}
// Dependencies: {}
# [doc = " Panics the current thread with the given message as the panic payload."] # [doc = ""] # [doc = " The message can be of any (`Any + Send`) type, not just strings."] # [doc = ""] # [doc = " The message is wrapped in a `Box<'static + Any + Send>`, which can be"] # [doc = " accessed later using [`PanicHookInfo::payload`]."] # [doc = ""] # [doc = " See the [`panic!`] macro for more information about panicking."] # [stable (feature = "panic_any" , since = "1.51.0")] # [inline] # [track_caller] # [cfg_attr (not (test) , rustc_diagnostic_item = "panic_any")] pub fn panic_any < M : 'static + Any + Send > (msg : M) -> ! { crate :: panicking :: begin_panic (msg) ; }
};
}
