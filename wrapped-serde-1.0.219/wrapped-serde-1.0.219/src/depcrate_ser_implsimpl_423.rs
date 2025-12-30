// Generated macro for impl_423 (impl)
macro_rules! Depcrate_ser_implsimpl_423 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_423"}
// Dependencies: {}
impl < T > Serialize for Reverse < T > where T : Serialize , { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } }
};
}
