// Generated macro for impl_400 (impl)
macro_rules! Depcrate_ser_implsimpl_400 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_400"}
// Dependencies: {}
impl < T > Serialize for Cell < T > where T : Serialize + Copy , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . get () . serialize (serializer) } }
};
}
