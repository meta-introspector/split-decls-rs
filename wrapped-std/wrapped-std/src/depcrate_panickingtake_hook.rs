// Generated macro for take_hook (function)
macro_rules! Depcrate_panickingtake_hook {
() => {
// Module: crate::panicking
// Provides: {"take_hook"}
// Dependencies: {}
# [doc = " Unregisters the current panic hook and returns it, registering the default hook"] # [doc = " in its place."] # [doc = ""] # [doc = " *See also the function [`set_hook`].*"] # [doc = ""] # [doc = " [`set_hook`]: ./fn.set_hook.html"] # [doc = ""] # [doc = " If the default hook is registered it will be returned, but remain registered."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if called from a panicking thread."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " The following will print \"Normal panic\":"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use std::panic;"] # [doc = ""] # [doc = " panic::set_hook(Box::new(|_| {"] # [doc = "     println!(\"Custom panic hook\");"] # [doc = " }));"] # [doc = ""] # [doc = " let _ = panic::take_hook();"] # [doc = ""] # [doc = " panic!(\"Normal panic\");"] # [doc = " ```"] # [must_use] # [stable (feature = "panic_hooks" , since = "1.10.0")] pub fn take_hook () -> Box < dyn Fn (& PanicHookInfo < '_ >) + 'static + Sync + Send > { if thread :: panicking () { panic ! ("cannot modify the panic hook from a panicking thread") ; } let mut hook = HOOK . write () . unwrap_or_else (PoisonError :: into_inner) ; let old_hook = mem :: take (& mut * hook) ; drop (hook) ; old_hook . into_box () }
};
}
