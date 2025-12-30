// Generated macro for impl_422 (impl)
macro_rules! Depcrate_ser_implsimpl_422 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_422"}
// Dependencies: {}
# [cfg (not (no_core_num_saturating))] impl < T > Serialize for Saturating < T > where T : Serialize , { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } }
};
}
