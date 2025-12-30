// Generated macro for impl_652 (impl)
macro_rules! Depcrate_streamimpl_652 {
() => {
// Module: crate::stream
// Provides: {"impl_652"}
// Dependencies: {}
impl < C1 : AsChar , C2 : AsChar + Clone > ContainsToken < C1 > for core :: ops :: RangeFrom < C2 > { # [inline (always)] fn contains_token (& self , token : C1) -> bool { let start = self . start . clone () . as_char () ; (start ..) . contains (& token . as_char ()) } }
};
}
