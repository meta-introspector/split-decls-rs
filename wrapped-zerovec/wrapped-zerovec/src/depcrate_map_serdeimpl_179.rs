// Generated macro for impl_179 (impl)
macro_rules! Depcrate_map_serdeimpl_179 {
() => {
// Module: crate::map::serde
// Provides: {"impl_179"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] # [cfg (feature = "serde")] impl < 'a , K , V > Serialize for ZeroMap < 'a , K , V > where K : ZeroMapKV < 'a > + Serialize + ? Sized + Ord , V : ZeroMapKV < 'a > + Serialize + ? Sized , K :: Container : Serialize , V :: Container : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if serializer . is_human_readable () { if let Some (k) = self . iter_keys () . next () { if ! K :: Container :: zvl_get_as_t (k , super :: serde_helpers :: is_num_or_string) { let mut seq = serializer . serialize_seq (Some (self . len ())) ? ; for (k , v) in self . iter () { K :: Container :: zvl_get_as_t (k , | k | { V :: Container :: zvl_get_as_t (v , | v | seq . serialize_element (& (k , v))) }) ? ; } return seq . end () ; } } let mut map = serializer . serialize_map (Some (self . len ())) ? ; for (k , v) in self . iter () { K :: Container :: zvl_get_as_t (k , | k | map . serialize_key (k)) ? ; V :: Container :: zvl_get_as_t (v , | v | map . serialize_value (v)) ? ; } map . end () } else { (& self . keys , & self . values) . serialize (serializer) } } }
};
}
