// Generated macro for impl_657 (impl)
macro_rules! Depcrate_streamimpl_657 {
() => {
// Module: crate::stream
// Provides: {"impl_657"}
// Dependencies: {}
impl < C : AsChar > ContainsToken < C > for & '_ [char] { # [inline] fn contains_token (& self , token : C) -> bool { let token = token . as_char () ; self . contains (& token) } }
};
}
