// Generated macro for impl_128 (impl)
macro_rules! Depcrate_commonimpl_128 {
() => {
// Module: crate::common
// Provides: {"impl_128"}
// Dependencies: {}
impl PartialEq < (Codepoint , Codepoint) > for Codepoints { fn eq (& self , other : & (Codepoint , Codepoint)) -> bool { match * self { Codepoints :: Single (ref x) => & (* x , * x) == other , Codepoints :: Range (ref x) => x == other , } } }
};
}
