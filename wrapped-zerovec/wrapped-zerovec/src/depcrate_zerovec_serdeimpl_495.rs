// Generated macro for impl_495 (impl)
macro_rules! Depcrate_zerovec_serdeimpl_495 {
() => {
// Module: crate::zerovec::serde
// Provides: {"impl_495"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] impl < T > Serialize for ZeroVec < '_ , T > where T : Serialize + AsULE , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if serializer . is_human_readable () { let mut seq = serializer . serialize_seq (Some (self . len ())) ? ; for value in self . iter () { seq . serialize_element (& value) ? ; } seq . end () } else { serializer . serialize_bytes (self . as_bytes ()) } } }
};
}
