// Generated macro for impl_293 (impl)
macro_rules! Depcrate_serimpl_293 {
() => {
// Module: crate::ser
// Provides: {"impl_293"}
// Dependencies: {}
impl serde :: ser :: SerializeTuple for SerializeTuple < '_ > { type Ok = Schema ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde :: Serialize + ? Sized , { let schema = value . serialize (Serializer { generator : self . generator , include_title : false , }) ? ; self . items . push (schema) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { let len = self . items . len () ; let mut schema = json_schema ! ({ "type" : "array" , "prefixItems" : self . items , "maxItems" : len , "minItems" : len , }) ; if ! self . title . is_empty () { schema . ensure_object () . insert ("title" . into () , self . title . into ()) ; } Ok (schema) } }
};
}
