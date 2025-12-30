// Generated macro for impl_654 (impl)
macro_rules! Depcrate_streamimpl_654 {
() => {
// Module: crate::stream
// Provides: {"impl_654"}
// Dependencies: {}
impl < C1 : AsChar , C2 : AsChar + Clone > ContainsToken < C1 > for core :: ops :: RangeToInclusive < C2 > { # [inline (always)] fn contains_token (& self , token : C1) -> bool { let end = self . end . clone () . as_char () ; (..= end) . contains (& token . as_char ()) } }
};
}
