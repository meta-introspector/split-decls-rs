// Generated macro for impl_12 (impl)
macro_rules! Depcrate_fieldsimpl_12 {
() => {
// Module: crate::fields
// Provides: {"impl_12"}
// Dependencies: {}
impl Serialize for SerializeFieldMap < '_ , Attributes < '_ > > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let len = self . 0 . metadata () . fields () . len () ; let serializer = serializer . serialize_map (Some (len)) ? ; let mut visitor = SerdeMapVisitor :: new (serializer) ; self . 0 . record (& mut visitor) ; visitor . finish () } }
};
}
