// Generated macro for impl_251 (impl)
macro_rules! Depcrate_textimpl_251 {
() => {
// Module: crate::text
// Provides: {"impl_251"}
// Dependencies: {}
impl Deadline { fn into_instant (self) -> Option < Instant > { match self { Deadline :: Absolute (instant) => Some (instant) , Deadline :: Relative (duration) => duration_to_deadline (duration) , } } }
};
}
