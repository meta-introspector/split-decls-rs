// Generated macro for impl_260 (impl)
macro_rules! Depcrate_schemaimpl_260 {
() => {
// Module: crate::schema
// Provides: {"impl_260"}
// Dependencies: {}
impl Serialize for Schema { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { ser :: OrderedKeywordWrapper :: from (& self . 0) . serialize (serializer) } }
};
}
