// Generated macro for impl_649 (impl)
macro_rules! Depcrate_streamimpl_649 {
() => {
// Module: crate::stream
// Provides: {"impl_649"}
// Dependencies: {}
impl < C , F : Fn (C) -> bool > ContainsToken < C > for F { # [inline (always)] fn contains_token (& self , token : C) -> bool { self (token) } }
};
}
