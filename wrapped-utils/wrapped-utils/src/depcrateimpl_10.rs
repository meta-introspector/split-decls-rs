// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < W , const N : usize > uWrite for LineBuffered < W , N > where W : uWrite , { type Error = W :: Error ; fn write_str (& mut self , mut s : & str) -> Result < () , W :: Error > { while let Some (pos) = s . as_bytes () . iter () . position (| b | * b == b'\n') { let line = s . get (.. pos + 1) . unwrap_or_else (| | unsafe { assume_unreachable ! () }) ; self . push_str (line) ? ; self . flush () ? ; s = s . get (pos + 1 ..) . unwrap_or_else (| | unsafe { assume_unreachable ! () }) ; } self . push_str (s) } }
};
}
