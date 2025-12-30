// Generated macro for impl_25 (impl)
macro_rules! Depcrate_consoleimpl_25 {
() => {
// Module: crate::console
// Provides: {"impl_25"}
// Dependencies: {}
impl HandleKind { fn handle (& self) -> HandleRef { match * self { HandleKind :: Stdout => HandleRef :: stdout () , HandleKind :: Stderr => HandleRef :: stderr () , } } }
};
}
