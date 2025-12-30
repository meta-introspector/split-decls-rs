// Generated macro for impl_421 (impl)
macro_rules! Depcrate_ser_implsimpl_421 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_421"}
// Dependencies: {}
impl < T > Serialize for Wrapping < T > where T : Serialize , { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } }
};
}
