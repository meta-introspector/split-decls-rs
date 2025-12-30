// Generated macro for impl_308 (impl)
macro_rules! Depcrate_flagimpl_308 {
() => {
// Module: crate::flag
// Provides: {"impl_308"}
// Dependencies: {}
impl ContextVerb { pub fn try_from_raw (raw : usize) -> Option < Self > { Some (match raw { 1 => Self :: Stop , 2 => Self :: Unstop , 3 => Self :: Interrupt , usize :: MAX => Self :: ForceKill , _ => return None , }) } }
};
}
