// Generated macro for impl_152 (impl)
macro_rules! Depcrate_tinyvecimpl_152 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_152"}
// Dependencies: {}
# [cfg (feature = "serde")] # [cfg_attr (docs_rs , doc (cfg (feature = "serde")))] impl < A : Array > Serialize for TinyVec < A > where A :: Item : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut seq = serializer . serialize_seq (Some (self . len ())) ? ; for element in self . iter () { seq . serialize_element (element) ? ; } seq . end () } }
};
}
