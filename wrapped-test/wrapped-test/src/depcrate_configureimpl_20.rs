// Generated macro for impl_20 (impl)
macro_rules! Depcrate_configureimpl_20 {
() => {
// Module: crate::configure
// Provides: {"impl_20"}
// Dependencies: {}
impl < T > Serialize for Readable < T > where T : ? Sized + Serialize , { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (Readable (serializer)) } }
};
}
