// Generated macro for impl_13 (impl)
macro_rules! Depcrate_fieldsimpl_13 {
() => {
// Module: crate::fields
// Provides: {"impl_13"}
// Dependencies: {}
impl Serialize for SerializeFieldMap < '_ , Record < '_ > > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let serializer = serializer . serialize_map (None) ? ; let mut visitor = SerdeMapVisitor :: new (serializer) ; self . 0 . record (& mut visitor) ; visitor . finish () } }
};
}
