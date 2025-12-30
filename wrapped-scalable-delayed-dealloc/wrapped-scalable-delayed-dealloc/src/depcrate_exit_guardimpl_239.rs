// Generated macro for impl_239 (impl)
macro_rules! Depcrate_exit_guardimpl_239 {
() => {
// Module: crate::exit_guard
// Provides: {"impl_239"}
// Dependencies: {}
impl < T , F : FnOnce (T) > ExitGuard < T , F > { # [doc = " Creates a new [`ExitGuard`] with the specified variables captured."] # [inline] pub (crate) const fn new (captured : T , drop_callback : F) -> Self { Self { drop_callback : ManuallyDrop :: new ((captured , drop_callback)) , } } # [doc = " Forgets the [`ExitGuard`] without invoking the drop callback."] # [inline] pub (crate) fn forget (mut self) { unsafe { ManuallyDrop :: drop (& mut self . drop_callback) ; } forget (self) ; } }
};
}
