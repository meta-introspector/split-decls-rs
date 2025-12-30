// Generated macro for panic_with_hook (function)
macro_rules! Depcrate_panickingpanic_with_hook {
() => {
// Module: crate::panicking
// Provides: {"panic_with_hook"}
// Dependencies: {}
# [doc = " Central point for dispatching panics."] # [doc = ""] # [doc = " Executes the primary logic for a panic, including checking for recursive"] # [doc = " panics, panic hooks, and finally dispatching to the panic runtime to either"] # [doc = " abort or unwind."] # [optimize (size)] fn panic_with_hook (payload : & mut dyn PanicPayload , location : & Location < '_ > , can_unwind : bool , force_no_backtrace : bool ,) -> ! { let must_abort = panic_count :: increase (true) ; if let Some (must_abort) = must_abort { match must_abort { panic_count :: MustAbort :: PanicInHook => { let message : & str = payload . as_str () . unwrap_or_default () ; rtprintpanic ! ("panicked at {location}:\n{message}\nthread panicked while processing panic. aborting.\n") ; } panic_count :: MustAbort :: AlwaysAbort => { rtprintpanic ! ("aborting due to panic at {location}:\n{payload}\n") ; } } crate :: process :: abort () ; } match * HOOK . read () . unwrap_or_else (PoisonError :: into_inner) { Hook :: Default if panic_output () . is_none () => { } Hook :: Default => { default_hook (& PanicHookInfo :: new (location , payload . get () , can_unwind , force_no_backtrace ,)) ; } Hook :: Custom (ref hook) => { hook (& PanicHookInfo :: new (location , payload . get () , can_unwind , force_no_backtrace)) ; } } panic_count :: finished_panic_hook () ; if ! can_unwind { rtprintpanic ! ("thread caused non-unwinding panic. aborting.\n") ; crate :: process :: abort () ; } rust_panic (payload) }
};
}
