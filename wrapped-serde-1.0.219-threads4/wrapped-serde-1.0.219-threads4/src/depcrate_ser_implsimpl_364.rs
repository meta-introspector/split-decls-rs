// Generated macro for impl_364 (impl)
macro_rules! Depcrate_ser_implsimpl_364 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_364"}
// Dependencies: {}
impl < T > Serialize for [T] where T : Serialize , { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_seq (self) } }
};
}
