// Generated macro for impl_620 (impl)
macro_rules! Depcrate_common_stateimpl_620 {
() => {
// Module: crate::common_state
// Provides: {"impl_620"}
// Dependencies: {}
impl Side { pub (crate) fn peer (& self) -> Self { match self { Self :: Client => Self :: Server , Self :: Server => Self :: Client , } } }
};
}
