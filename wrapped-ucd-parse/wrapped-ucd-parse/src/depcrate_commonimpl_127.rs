// Generated macro for impl_127 (impl)
macro_rules! Depcrate_commonimpl_127 {
() => {
// Module: crate::common
// Provides: {"impl_127"}
// Dependencies: {}
impl PartialEq < (u32 , u32) > for Codepoints { fn eq (& self , other : & (u32 , u32)) -> bool { match * self { Codepoints :: Single (ref x) => & (x . value () , x . value ()) == other , Codepoints :: Range (ref x) => x == other , } } }
};
}
