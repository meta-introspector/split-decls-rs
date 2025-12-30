// Generated macro for impl_661 (impl)
macro_rules! Depcrate_streamimpl_661 {
() => {
// Module: crate::stream
// Provides: {"impl_661"}
// Dependencies: {}
impl < const LEN : usize , C : AsChar > ContainsToken < C > for [char ; LEN] { # [inline] fn contains_token (& self , token : C) -> bool { let token = token . as_char () ; self . contains (& token) } }
};
}
