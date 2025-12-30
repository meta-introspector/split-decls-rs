// Generated macro for impl_312 (impl)
macro_rules! Depcrate_flagimpl_312 {
() => {
// Module: crate::flag
// Provides: {"impl_312"}
// Dependencies: {}
impl SchemeSocketCall { pub fn try_from_raw (raw : usize) -> Option < Self > { Some (match raw { 0 => Self :: ObtainFd , 1 => Self :: MoveFd , _ => return None , }) } }
};
}
