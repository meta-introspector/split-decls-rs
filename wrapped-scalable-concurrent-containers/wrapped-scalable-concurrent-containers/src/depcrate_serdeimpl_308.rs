// Generated macro for impl_308 (impl)
macro_rules! Depcrate_serdeimpl_308 {
() => {
// Module: crate::serde
// Provides: {"impl_308"}
// Dependencies: {}
impl < K , V , H > Serialize for HashCache < K , V , H > where K : Eq + Hash + Serialize , V : Serialize , H : BuildHasher , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let capacity_range = self . capacity_range () ; let mut map = serializer . serialize_map (Some (* capacity_range . end ())) ? ; let mut error = None ; self . iter_sync (| k , v | { if error . is_none () { if let Err (e) = map . serialize_entry (k , v) { error . replace (e) ; } } true }) ; if let Some (e) = error { return Err (e) ; } map . end () } }
};
}
