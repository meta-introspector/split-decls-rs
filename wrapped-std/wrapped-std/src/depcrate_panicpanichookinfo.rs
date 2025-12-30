// Generated macro for PanicHookInfo (struct)
macro_rules! Depcrate_panicPanicHookInfo {
() => {
// Module: crate::panic
// Provides: {"PanicHookInfo"}
// Dependencies: {}
# [doc = " A struct providing information about a panic."] # [doc = ""] # [doc = " `PanicHookInfo` structure is passed to a panic hook set by the [`set_hook`] function."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use std::panic;"] # [doc = ""] # [doc = " panic::set_hook(Box::new(|panic_info| {"] # [doc = "     println!(\"panic occurred: {panic_info}\");"] # [doc = " }));"] # [doc = ""] # [doc = " panic!(\"critical system failure\");"] # [doc = " ```"] # [doc = ""] # [doc = " [`set_hook`]: ../../std/panic/fn.set_hook.html"] # [stable (feature = "panic_hook_info" , since = "1.81.0")] # [derive (Debug)] pub struct PanicHookInfo < 'a > { payload : & 'a (dyn Any + Send) , location : & 'a Location < 'a > , can_unwind : bool , force_no_backtrace : bool , }
};
}
