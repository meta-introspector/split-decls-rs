// Generated macro for impl_293 (impl)
macro_rules! Depcrate_serdeimpl_293 {
() => {
// Module: crate::serde
// Provides: {"impl_293"}
// Dependencies: {}
impl < K , V , H > Serialize for HashMap < K , V , H > where K : Eq + Hash + Serialize , V : Serialize , H : BuildHasher , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut map = serializer . serialize_map (Some (self . len ())) ? ; let mut error = None ; self . iter_sync (| k , v | { if error . is_none () { if let Err (e) = map . serialize_entry (k , v) { error . replace (e) ; } } true }) ; if let Some (e) = error { return Err (e) ; } map . end () } }
};
}
