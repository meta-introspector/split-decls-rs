// Generated macro for impl_303 (impl)
macro_rules! Depcrate_serdeimpl_303 {
() => {
// Module: crate::serde
// Provides: {"impl_303"}
// Dependencies: {}
impl < K , V , H > Serialize for HashIndex < K , V , H > where K : Eq + Hash + Serialize , V : Serialize , H : BuildHasher , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut map = serializer . serialize_map (Some (self . len ())) ? ; let mut error = None ; self . iter (& Guard :: new ()) . any (| (k , v) | { if let Err (e) = map . serialize_entry (k , v) { error . replace (e) ; true } else { false } }) ; if let Some (e) = error { return Err (e) ; } map . end () } }
};
}
