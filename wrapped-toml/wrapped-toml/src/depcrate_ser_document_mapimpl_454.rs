// Generated macro for impl_454 (impl)
macro_rules! Depcrate_ser_document_mapimpl_454 {
() => {
// Module: crate::ser::document::map
// Provides: {"impl_454"}
// Dependencies: {}
impl < 'd > serde_core :: ser :: SerializeStructVariant for SerializeDocumentTable < 'd > { type Ok = & 'd mut Buffer ; type Error = Error ; # [inline] fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { serde_core :: ser :: SerializeStruct :: serialize_field (self , key , value) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { self . end () } }
};
}
