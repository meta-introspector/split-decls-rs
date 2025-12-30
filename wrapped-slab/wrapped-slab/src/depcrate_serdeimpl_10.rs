// Generated macro for impl_10 (impl)
macro_rules! Depcrate_serdeimpl_10 {
() => {
// Module: crate::serde
// Provides: {"impl_10"}
// Dependencies: {}
impl < T > Serialize for Slab < T > where T : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut map_serializer = serializer . serialize_map (Some (self . len ())) ? ; for (key , value) in self { map_serializer . serialize_key (& key) ? ; map_serializer . serialize_value (value) ? ; } map_serializer . end () } }
};
}
