// Generated macro for impl_11 (impl)
macro_rules! Depcrate_fieldsimpl_11 {
() => {
// Module: crate::fields
// Provides: {"impl_11"}
// Dependencies: {}
impl Serialize for SerializeFieldMap < '_ , Event < '_ > > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let len = self . 0 . fields () . count () ; let serializer = serializer . serialize_map (Some (len)) ? ; let mut visitor = SerdeMapVisitor :: new (serializer) ; self . 0 . record (& mut visitor) ; visitor . finish () } }
};
}
