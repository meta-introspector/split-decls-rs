// Generated macro for impl_114 (impl)
macro_rules! Depcrate_serimpl_114 {
() => {
// Module: crate::ser
// Provides: {"impl_114"}
// Dependencies: {}
impl < T > Serialize for Option < T > where T : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { struct AsBytes < T > (T) ; impl < T > serde :: Serialize for AsBytes < T > where T : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } } match self { Some (b) => serializer . serialize_some (& AsBytes (b)) , None => serializer . serialize_none () , } } }
};
}
