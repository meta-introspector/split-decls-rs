// Generated macro for impl_656 (impl)
macro_rules! Depcrate_streamimpl_656 {
() => {
// Module: crate::stream
// Provides: {"impl_656"}
// Dependencies: {}
impl < C : AsChar > ContainsToken < C > for & '_ [u8] { # [inline] fn contains_token (& self , token : C) -> bool { let token = token . as_char () ; self . iter () . any (| t | t . as_char () == token) } }
};
}
