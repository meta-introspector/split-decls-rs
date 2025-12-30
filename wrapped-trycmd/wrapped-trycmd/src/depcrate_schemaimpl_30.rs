// Generated macro for impl_30 (impl)
macro_rules! Depcrate_schemaimpl_30 {
() => {
// Module: crate::schema
// Provides: {"impl_30"}
// Dependencies: {}
impl serde :: ser :: Serialize for JoinedArgs { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { serializer . serialize_str (& self . to_string ()) } }
};
}
