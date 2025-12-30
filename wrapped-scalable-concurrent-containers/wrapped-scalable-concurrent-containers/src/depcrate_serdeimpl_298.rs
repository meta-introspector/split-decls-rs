// Generated macro for impl_298 (impl)
macro_rules! Depcrate_serdeimpl_298 {
() => {
// Module: crate::serde
// Provides: {"impl_298"}
// Dependencies: {}
impl < K , H > Serialize for HashSet < K , H > where K : Eq + Hash + Serialize , H : BuildHasher , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut seq = serializer . serialize_seq (Some (self . len ())) ? ; let mut error = None ; self . iter_sync (| k | { if error . is_none () { if let Err (e) = seq . serialize_element (k) { error . replace (e) ; } } true }) ; if let Some (e) = error { return Err (e) ; } seq . end () } }
};
}
