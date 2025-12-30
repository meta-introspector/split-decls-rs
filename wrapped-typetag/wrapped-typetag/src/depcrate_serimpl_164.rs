// Generated macro for impl_164 (impl)
macro_rules! Depcrate_serimpl_164 {
() => {
// Module: crate::ser
// Provides: {"impl_164"}
// Dependencies: {}
impl < S > InternallyTaggedSerializer < S > where S : Serializer , { fn serialize_default < T > (self , value : & T) -> Result < S :: Ok , S :: Error > where T : ? Sized + Serialize , { let mut map = self . delegate . serialize_map (Some (2)) ? ; map . serialize_entry (self . tag , self . variant) ? ; map . serialize_entry (DEFAULT_KEY , value) ? ; map . end () } }
};
}
