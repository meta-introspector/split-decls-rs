// Generated macro for impl_348 (impl)
macro_rules! Depcrateimpl_348 {
() => {
// Module: crate
// Provides: {"impl_348"}
// Dependencies: {}
impl < T , E > UnwrapThrowExt < T > for Result < T , E > where E : core :: fmt :: Debug , { fn unwrap_throw (self) -> T { const MSG : & str = "called `Result::unwrap_throw()` on an `Err` value" ; if cfg ! (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none"))) { match self { Ok (val) => val , Err (err) => { if cfg ! (debug_assertions) { let loc = core :: panic :: Location :: caller () ; let msg = alloc :: format ! ("{} ({}:{}:{}): {:?}" , MSG , loc . file () , loc . line () , loc . column () , err) ; throw_str (& msg) } else { throw_str (MSG) } } } } else { self . expect (MSG) } } fn expect_throw (self , message : & str) -> T { if cfg ! (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none"))) { match self { Ok (val) => val , Err (err) => { if cfg ! (debug_assertions) { let loc = core :: panic :: Location :: caller () ; let msg = alloc :: format ! ("{} ({}:{}:{}): {:?}" , message , loc . file () , loc . line () , loc . column () , err) ; throw_str (& msg) } else { throw_str (message) } } } } else { self . expect (message) } } }
};
}
