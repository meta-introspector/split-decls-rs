// Generated macro for impl_295 (impl)
macro_rules! Depcrate_serimpl_295 {
() => {
// Module: crate::ser
// Provides: {"impl_295"}
// Dependencies: {}
impl serde :: ser :: SerializeMap for SerializeMap < '_ > { type Ok = Schema ; type Error = Error ; fn serialize_key < T > (& mut self , key : & T) -> Result < () , Self :: Error > where T : serde :: Serialize + ? Sized , { let json = serde_json :: to_string (key) ? ; self . current_key = Some (json . trim_start_matches ('"') . trim_end_matches ('"') . to_string () ,) ; Ok (()) } fn serialize_value < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde :: Serialize + ? Sized , { let key = self . current_key . take () . unwrap_or_default () ; let schema = value . serialize (Serializer { generator : self . generator , include_title : false , }) ? ; self . properties . insert (key , schema . into ()) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { let mut schema = json_schema ! ({ "type" : "object" , "properties" : self . properties , }) ; if ! self . title . is_empty () { schema . ensure_object () . insert ("title" . into () , self . title . into ()) ; } Ok (schema) } }
};
}
