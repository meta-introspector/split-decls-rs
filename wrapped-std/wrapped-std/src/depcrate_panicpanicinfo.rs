// Generated macro for PanicInfo (type)
macro_rules! Depcrate_panicPanicInfo {
() => {
// Module: crate::panic
// Provides: {"PanicInfo"}
// Dependencies: {}
# [stable (feature = "panic_hooks" , since = "1.10.0")] # [deprecated (since = "1.82.0" , note = "use `PanicHookInfo` instead" , suggestion = "std::panic::PanicHookInfo")] # [doc = " A struct providing information about a panic."] # [doc = ""] # [doc = " `PanicInfo` has been renamed to [`PanicHookInfo`] to avoid confusion with"] # [doc = " [`core::panic::PanicInfo`]."] pub type PanicInfo < 'a > = PanicHookInfo < 'a > ;
};
}
