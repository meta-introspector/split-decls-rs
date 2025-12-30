// Generated macro for impl_651 (impl)
macro_rules! Depcrate_streamimpl_651 {
() => {
// Module: crate::stream
// Provides: {"impl_651"}
// Dependencies: {}
impl < C1 : AsChar , C2 : AsChar + Clone > ContainsToken < C1 > for core :: ops :: RangeInclusive < C2 > { # [inline (always)] fn contains_token (& self , token : C1) -> bool { let start = self . start () . clone () . as_char () ; let end = self . end () . clone () . as_char () ; (start ..= end) . contains (& token . as_char ()) } }
};
}
