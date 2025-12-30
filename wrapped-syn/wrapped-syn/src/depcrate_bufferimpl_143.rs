// Generated macro for impl_143 (impl)
macro_rules! Depcrate_bufferimpl_143 {
() => {
// Module: crate::buffer
// Provides: {"impl_143"}
// Dependencies: {}
impl < 'a > PartialOrd for Cursor < 'a > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { if same_buffer (* self , * other) { Some (cmp_assuming_same_buffer (* self , * other)) } else { None } } }
};
}
