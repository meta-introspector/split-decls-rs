// Generated macro for impl_385 (impl)
macro_rules! Depcrate_return_modeimpl_385 {
() => {
// Module: crate::return_mode
// Provides: {"impl_385"}
// Dependencies: {}
impl < T , E > SalsaAsRef for Result < T , E > { type AsRef < 'a > = Result < & 'a T , & 'a E > where Self : 'a ; fn as_ref (& self) -> Self :: AsRef < '_ > { self . as_ref () } }
};
}
