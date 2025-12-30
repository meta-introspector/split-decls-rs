// Generated macro for impl_627 (impl)
macro_rules! Depcrate_common_stateimpl_627 {
() => {
// Module: crate::common_state
// Provides: {"impl_627"}
// Dependencies: {}
impl KxState { pub (crate) fn complete (& mut self) { debug_assert ! (matches ! (self , Self :: Start (_))) ; if let Self :: Start (group) = self { * self = Self :: Complete (* group) ; } } }
};
}
