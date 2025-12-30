// Generated macro for impl_659 (impl)
macro_rules! Depcrate_streamimpl_659 {
() => {
// Module: crate::stream
// Provides: {"impl_659"}
// Dependencies: {}
impl < const LEN : usize , C : AsChar > ContainsToken < C > for & '_ [char ; LEN] { # [inline] fn contains_token (& self , token : C) -> bool { let token = token . as_char () ; self . contains (& token) } }
};
}
