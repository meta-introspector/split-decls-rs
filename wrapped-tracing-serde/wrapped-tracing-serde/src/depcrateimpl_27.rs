// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl Serialize for SerializeAttributes < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut serializer = serializer . serialize_struct ("Attributes" , 3) ? ; serializer . serialize_field ("metadata" , & SerializeMetadata (self . 0 . metadata ())) ? ; serializer . serialize_field ("parent" , & self . 0 . parent () . map (SerializeId)) ? ; serializer . serialize_field ("is_root" , & self . 0 . is_root ()) ? ; let mut visitor = SerdeStructVisitor { serializer , state : Ok (()) , } ; self . 0 . record (& mut visitor) ; visitor . finish () } }
};
}
