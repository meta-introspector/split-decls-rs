// Generated macro for impl_424 (impl)
macro_rules! Depcrate_ser_document_array_of_tablesimpl_424 {
() => {
// Module: crate::ser::document::array_of_tables
// Provides: {"impl_424"}
// Dependencies: {}
impl < 'd > serde_core :: ser :: SerializeSeq for SerializeArrayOfTablesSerializer < 'd > { type Ok = & 'd mut Buffer ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { let child = self . buf . element_table (& mut self . parent , self . key . clone ()) ; let value_serializer = Serializer :: with_table (self . buf , child , self . style) ; value . serialize (value_serializer) ? ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . end () } }
};
}
