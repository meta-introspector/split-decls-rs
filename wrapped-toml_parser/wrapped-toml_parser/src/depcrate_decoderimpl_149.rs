// Generated macro for impl_149 (impl)
macro_rules! Depcrate_decoderimpl_149 {
() => {
// Module: crate::decoder
// Provides: {"impl_149"}
// Dependencies: {}
impl < 's > StringBuilder < 's > for & 's str { fn clear (& mut self) { * self = & self [0 .. 0] ; } fn push_str (& mut self , append : & 's str) -> bool { if self . is_empty () { * self = append ; true } else { false } } fn push_char (& mut self , _append : char) -> bool { false } }
};
}
