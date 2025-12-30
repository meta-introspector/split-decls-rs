// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl Serialize for SerializeEvent < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut serializer = serializer . serialize_struct ("Event" , 2) ? ; serializer . serialize_field ("metadata" , & SerializeMetadata (self . 0 . metadata ())) ? ; let mut visitor = SerdeStructVisitor { serializer , state : Ok (()) , } ; self . 0 . record (& mut visitor) ; visitor . finish () } }
};
}
