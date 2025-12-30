// Generated macro for impl_15 (impl)
macro_rules! Depcrate_winimpl_15 {
() => {
// Module: crate::win
// Provides: {"impl_15"}
// Dependencies: {}
impl HandleKind { fn handle (& self) -> winutil :: HandleRef { match * self { HandleKind :: Stdout => winutil :: HandleRef :: stdout () , HandleKind :: Stderr => winutil :: HandleRef :: stderr () , } } }
};
}
