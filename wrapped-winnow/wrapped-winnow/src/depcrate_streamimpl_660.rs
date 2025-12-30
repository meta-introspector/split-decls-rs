// Generated macro for impl_660 (impl)
macro_rules! Depcrate_streamimpl_660 {
() => {
// Module: crate::stream
// Provides: {"impl_660"}
// Dependencies: {}
impl < const LEN : usize , C : AsChar > ContainsToken < C > for [u8 ; LEN] { # [inline] fn contains_token (& self , token : C) -> bool { let token = token . as_char () ; self . iter () . any (| t | t . as_char () == token) } }
};
}
