// Generated macro for impl_142 (impl)
macro_rules! Depcrate_bufferimpl_142 {
() => {
// Module: crate::buffer
// Provides: {"impl_142"}
// Dependencies: {}
impl < 'a > PartialOrd for Cursor < 'a > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { if same_buffer (* self , * other) { Some (cmp_assuming_same_buffer (* self , * other)) } else { None } } }
};
}
