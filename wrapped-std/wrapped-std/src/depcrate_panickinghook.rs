// Generated macro for Hook (enum)
macro_rules! Depcrate_panickingHook {
() => {
// Module: crate::panicking
// Provides: {"Hook"}
// Dependencies: {}
# [derive (Default)] enum Hook { # [default] Default , Custom (Box < dyn Fn (& PanicHookInfo < '_ >) + 'static + Sync + Send >) , }
};
}
