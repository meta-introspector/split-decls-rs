// Generated macro for impl_387 (impl)
macro_rules! Depcrate_return_modeimpl_387 {
() => {
// Module: crate::return_mode
// Provides: {"impl_387"}
// Dependencies: {}
impl < T : Deref > SalsaAsDeref for Option < T > { type AsDeref < 'a > = Option < & 'a T :: Target > where Self : 'a ; fn as_deref (& self) -> Self :: AsDeref < '_ > { self . as_deref () } }
};
}
