// Generated macro for impl_113 (impl)
macro_rules! Depcrate_serimpl_113 {
() => {
// Module: crate::ser
// Provides: {"impl_113"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < T > Serialize for Box < T > where T : ? Sized + Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { (* * self) . serialize (serializer) } }
};
}
