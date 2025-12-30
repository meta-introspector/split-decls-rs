// Generated macro for impl_292 (impl)
macro_rules! Depcrate_serimpl_292 {
() => {
// Module: crate::ser
// Provides: {"impl_292"}
// Dependencies: {}
impl serde :: ser :: SerializeSeq for SerializeSeq < '_ > { type Ok = Schema ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde :: Serialize + ? Sized , { if self . items . first () == Some (& true . into ()) { return Ok (()) ; } let schema = value . serialize (Serializer { generator : self . generator , include_title : false , }) ? ; if schema == true { self . items = vec ! [schema] ; } else if ! self . items . contains (& schema) { self . items . push (schema) ; } Ok (()) } fn end (mut self) -> Result < Self :: Ok , Self :: Error > { let items = match self . items . len () { 0 => true . into () , 1 => self . items . remove (0) , _ => json_schema ! ({ "anyOf" : self . items }) , } ; Ok (json_schema ! ({ "type" : "array" , "items" : items })) } }
};
}
