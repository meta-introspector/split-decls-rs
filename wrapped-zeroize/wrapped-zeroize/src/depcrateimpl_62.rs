// Generated macro for impl_62 (impl)
macro_rules! Depcrateimpl_62 {
() => {
// Module: crate
// Provides: {"impl_62"}
// Dependencies: {}
impl < Z : Zeroize + Clone > Clone for Zeroizing < Z > { # [inline (always)] fn clone (& self) -> Self { Self (self . 0 . clone ()) } # [inline (always)] fn clone_from (& mut self , source : & Self) { self . 0 . zeroize () ; self . 0 . clone_from (& source . 0) ; } }
};
}
