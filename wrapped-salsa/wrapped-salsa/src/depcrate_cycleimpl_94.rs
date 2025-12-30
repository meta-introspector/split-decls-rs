// Generated macro for impl_94 (impl)
macro_rules! Depcrate_cycleimpl_94 {
() => {
// Module: crate::cycle
// Provides: {"impl_94"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl serde :: Serialize for CycleHeads { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { use serde :: ser :: SerializeSeq ; let mut seq = serializer . serialize_seq (None) ? ; for e in self { if e . removed . load (Ordering :: Relaxed) { continue ; } seq . serialize_element (e) ? ; } seq . end () } }
};
}
