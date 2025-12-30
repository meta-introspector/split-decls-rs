// Generated macro for impl_21 (impl)
macro_rules! Depcrate_configureimpl_21 {
() => {
// Module: crate::configure
// Provides: {"impl_21"}
// Dependencies: {}
impl < T > Serialize for Compact < T > where T : ? Sized + Serialize , { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (Compact (serializer)) } }
};
}
