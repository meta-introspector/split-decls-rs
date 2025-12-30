// Generated macro for impl_1574 (impl)
macro_rules! Depcrateimpl_1574 {
() => {
// Module: crate
// Provides: {"impl_1574"}
// Dependencies: {}
impl < 'b , T : Write + 'b > Drop for Session < 'b , T > { fn drop (& mut self) { if let Some (ref mut out) = self . out { let _ = self . emitter . emit_footer (out) ; } } }
};
}
