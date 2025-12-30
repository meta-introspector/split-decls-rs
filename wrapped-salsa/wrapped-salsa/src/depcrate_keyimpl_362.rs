// Generated macro for impl_362 (impl)
macro_rules! Depcrate_keyimpl_362 {
() => {
// Module: crate::key
// Provides: {"impl_362"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl serde :: Serialize for DatabaseKeyIndex { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serde :: Serialize :: serialize (& (self . key_index , self . ingredient_index) , serializer) } }
};
}
