// Generated macro for impl_125 (impl)
macro_rules! Depcrate_commonimpl_125 {
() => {
// Module: crate::common
// Provides: {"impl_125"}
// Dependencies: {}
impl PartialEq < u32 > for Codepoints { fn eq (& self , other : & u32) -> bool { match * self { Codepoints :: Single (ref x) => x == other , Codepoints :: Range (ref x) => x == & (* other , * other) , } } }
};
}
