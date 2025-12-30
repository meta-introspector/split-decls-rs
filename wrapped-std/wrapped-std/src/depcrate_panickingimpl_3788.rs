// Generated macro for impl_3788 (impl)
macro_rules! Depcrate_panickingimpl_3788 {
() => {
// Module: crate::panicking
// Provides: {"impl_3788"}
// Dependencies: {}
impl Hook { # [inline] fn into_box (self) -> Box < dyn Fn (& PanicHookInfo < '_ >) + 'static + Sync + Send > { match self { Hook :: Default => Box :: new (default_hook) , Hook :: Custom (hook) => hook , } } }
};
}
