// Generated macro for impl_388 (impl)
macro_rules! Depcrate_return_modeimpl_388 {
() => {
// Module: crate::return_mode
// Provides: {"impl_388"}
// Dependencies: {}
impl < T : Deref , E > SalsaAsDeref for Result < T , E > { type AsDeref < 'a > = Result < & 'a T :: Target , & 'a E > where Self : 'a ; fn as_deref (& self) -> Self :: AsDeref < '_ > { self . as_deref () } }
};
}
