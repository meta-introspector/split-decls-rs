// Generated macro for impl_90 (impl)
macro_rules! Depcrate_cycleimpl_90 {
() => {
// Module: crate::cycle
// Provides: {"impl_90"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl serde :: Serialize for AtomicIterationCount { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { self . load () . serialize (serializer) } }
};
}
