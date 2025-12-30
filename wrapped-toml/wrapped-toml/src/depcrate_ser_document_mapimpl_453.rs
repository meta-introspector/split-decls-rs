// Generated macro for impl_453 (impl)
macro_rules! Depcrate_ser_document_mapimpl_453 {
() => {
// Module: crate::ser::document::map
// Provides: {"impl_453"}
// Dependencies: {}
impl < 'd > serde_core :: ser :: SerializeStruct for SerializeDocumentTable < 'd > { type Ok = & 'd mut Buffer ; type Error = Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { match SerializationStrategy :: from (value) { SerializationStrategy :: Value => { let dst = self . table . body_mut () ; dst . key (key) ? ; dst . space () ? ; dst . keyval_sep () ? ; dst . space () ? ; let value_serializer = ValueSerializer :: with_style (dst , self . style) ; let dst = value . serialize (value_serializer) ? ; dst . newline () ? ; } SerializationStrategy :: ArrayOfTables => { self . table . has_children (true) ; let value_serializer = ArrayOfTablesSerializer :: new (self . buf , self . table . clone () , key . to_owned () , self . style ,) ; value . serialize (value_serializer) ? ; } SerializationStrategy :: Table | SerializationStrategy :: Unknown => { let child = self . buf . child_table (& mut self . table , key . to_owned ()) ; let value_serializer = Serializer :: with_table (self . buf , child , self . style) ; value . serialize (value_serializer) ? ; } SerializationStrategy :: Skip => { } } Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . end () } }
};
}
