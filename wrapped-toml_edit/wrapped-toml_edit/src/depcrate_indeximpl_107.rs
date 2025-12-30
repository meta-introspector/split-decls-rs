// Generated macro for impl_107 (impl)
macro_rules! Depcrate_indeximpl_107 {
() => {
// Module: crate::index
// Provides: {"impl_107"}
// Dependencies: {}
impl < T : ? Sized > Index for & T where T : Index , { fn index < 'v > (& self , v : & 'v Item) -> Option < & 'v Item > { (* * self) . index (v) } fn index_mut < 'v > (& self , v : & 'v mut Item) -> Option < & 'v mut Item > { (* * self) . index_mut (v) } }
};
}
