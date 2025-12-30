// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl < W > uWrite for WriteAdapter < W > where W : fmt :: Write , { type Error = fmt :: Error ; fn write_char (& mut self , c : char) -> Result < () , Self :: Error > { self . 0 . write_char (c) } fn write_str (& mut self , s : & str) -> Result < () , Self :: Error > { self . 0 . write_str (s) } }
};
}
