// Generated macro for impl_426 (impl)
macro_rules! Depcrate_ser_document_array_of_tablesimpl_426 {
() => {
// Module: crate::ser::document::array_of_tables
// Provides: {"impl_426"}
// Dependencies: {}
impl < 'd > serde_core :: ser :: SerializeTupleStruct for SerializeArrayOfTablesSerializer < 'd > { type Ok = & 'd mut Buffer ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { serde_core :: ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { serde_core :: ser :: SerializeSeq :: end (self) } }
};
}
