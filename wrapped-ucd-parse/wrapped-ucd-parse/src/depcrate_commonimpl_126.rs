// Generated macro for impl_126 (impl)
macro_rules! Depcrate_commonimpl_126 {
() => {
// Module: crate::common
// Provides: {"impl_126"}
// Dependencies: {}
impl PartialEq < Codepoint > for Codepoints { fn eq (& self , other : & Codepoint) -> bool { match * self { Codepoints :: Single (ref x) => x == other , Codepoints :: Range (ref x) => x == & (* other , * other) , } } }
};
}
