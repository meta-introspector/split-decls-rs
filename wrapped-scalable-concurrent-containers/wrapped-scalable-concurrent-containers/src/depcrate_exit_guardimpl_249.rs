// Generated macro for impl_249 (impl)
macro_rules! Depcrate_exit_guardimpl_249 {
() => {
// Module: crate::exit_guard
// Provides: {"impl_249"}
// Dependencies: {}
impl < T , F : FnOnce (T) > ExitGuard < T , F > { # [doc = " Creates a new [`ExitGuard`] with the specified variables captured."] # [inline] pub (crate) const fn new (captured : T , drop_callback : F) -> Self { Self { drop_callback : ManuallyDrop :: new ((captured , drop_callback)) , } } }
};
}
