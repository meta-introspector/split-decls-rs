// Generated macro for impl_347 (impl)
macro_rules! Depcrateimpl_347 {
() => {
// Module: crate
// Provides: {"impl_347"}
// Dependencies: {}
impl < T > UnwrapThrowExt < T > for Option < T > { fn unwrap_throw (self) -> T { const MSG : & str = "called `Option::unwrap_throw()` on a `None` value" ; if cfg ! (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none"))) { if let Some (val) = self { val } else if cfg ! (debug_assertions) { let loc = core :: panic :: Location :: caller () ; let msg = alloc :: format ! ("{} ({}:{}:{})" , MSG , loc . file () , loc . line () , loc . column () ,) ; throw_str (& msg) } else { throw_str (MSG) } } else { self . expect (MSG) } } fn expect_throw (self , message : & str) -> T { if cfg ! (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none"))) { if let Some (val) = self { val } else if cfg ! (debug_assertions) { let loc = core :: panic :: Location :: caller () ; let msg = alloc :: format ! ("{} ({}:{}:{})" , message , loc . file () , loc . line () , loc . column () ,) ; throw_str (& msg) } else { throw_str (message) } } else { self . expect (message) } } }
};
}
