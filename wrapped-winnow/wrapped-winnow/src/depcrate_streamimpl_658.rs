// Generated macro for impl_658 (impl)
macro_rules! Depcrate_streamimpl_658 {
() => {
// Module: crate::stream
// Provides: {"impl_658"}
// Dependencies: {}
impl < const LEN : usize , C : AsChar > ContainsToken < C > for & '_ [u8 ; LEN] { # [inline] fn contains_token (& self , token : C) -> bool { let token = token . as_char () ; self . iter () . any (| t | t . as_char () == token) } }
};
}
