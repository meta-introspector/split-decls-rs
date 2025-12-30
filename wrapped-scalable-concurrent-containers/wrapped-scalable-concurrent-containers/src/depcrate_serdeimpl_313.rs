// Generated macro for impl_313 (impl)
macro_rules! Depcrate_serdeimpl_313 {
() => {
// Module: crate::serde
// Provides: {"impl_313"}
// Dependencies: {}
impl < K , V > Serialize for TreeIndex < K , V > where K : 'static + Clone + Ord + Serialize , V : 'static + Clone + Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut map = serializer . serialize_map (Some (self . len ())) ? ; let mut error = None ; self . iter (& Guard :: new ()) . any (| (k , v) | { if let Err (e) = map . serialize_entry (k , v) { error . replace (e) ; true } else { false } }) ; if let Some (e) = error { return Err (e) ; } map . end () } }
};
}
