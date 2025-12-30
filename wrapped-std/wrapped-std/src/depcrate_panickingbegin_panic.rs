// Generated macro for begin_panic (function)
macro_rules! Depcrate_panickingbegin_panic {
() => {
// Module: crate::panicking
// Provides: {"begin_panic"}
// Dependencies: {}
# [doc = " This is the entry point of panicking for the non-format-string variants of"] # [doc = " panic!() and assert!(). In particular, this is the only entry point that supports"] # [doc = " arbitrary payloads, not just format strings."] # [unstable (feature = "libstd_sys_internals" , reason = "used by the panic! macro" , issue = "none")] # [cfg_attr (not (any (test , doctest)) , lang = "begin_panic")] # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold , optimize (size))] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [track_caller] # [rustc_do_not_const_check] pub const fn begin_panic < M : Any + Send > (msg : M) -> ! { if cfg ! (feature = "panic_immediate_abort") { intrinsics :: abort () } struct Payload < A > { inner : Option < A > , } unsafe impl < A : Send + 'static > PanicPayload for Payload < A > { fn take_box (& mut self) -> * mut (dyn Any + Send) { let data = match self . inner . take () { Some (a) => Box :: new (a) as Box < dyn Any + Send > , None => process :: abort () , } ; Box :: into_raw (data) } fn get (& mut self) -> & (dyn Any + Send) { match self . inner { Some (ref a) => a , None => process :: abort () , } } } impl < A : 'static > fmt :: Display for Payload < A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . inner { Some (a) => f . write_str (payload_as_str (a)) , None => process :: abort () , } } } let loc = Location :: caller () ; crate :: sys :: backtrace :: __rust_end_short_backtrace (move | | { panic_with_hook (& mut Payload { inner : Some (msg) } , loc , true , false ,) }) }
};
}
