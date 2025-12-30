// Generated macro for impl_101 (impl)
macro_rules! Depcrate_serimpl_101 {
() => {
// Module: crate::ser
// Provides: {"impl_101"}
// Dependencies: {}
impl < 'input , 'output , Target > ser :: SerializeMap for MapSerializer < 'input , 'output , Target > where Target : 'output + UrlEncodedTarget , { type Ok = & 'output mut UrlEncodedSerializer < 'input , Target > ; type Error = Error ; fn serialize_entry < K : ? Sized + ser :: Serialize , V : ? Sized + ser :: Serialize , > (& mut self , key : & K , value : & V ,) -> Result < () , Error > { let key_sink = key :: KeySink :: new (| key | { let value_sink = value :: ValueSink :: new (self . urlencoder , & key) ; value . serialize (part :: PartSerializer :: new (value_sink)) ? ; self . key = None ; Ok (()) }) ; let entry_serializer = part :: PartSerializer :: new (key_sink) ; key . serialize (entry_serializer) } fn serialize_key < T : ? Sized + ser :: Serialize > (& mut self , key : & T ,) -> Result < () , Error > { let key_sink = key :: KeySink :: new (| key | Ok (key . into ())) ; let key_serializer = part :: PartSerializer :: new (key_sink) ; self . key = Some (key . serialize (key_serializer) ?) ; Ok (()) } fn serialize_value < T : ? Sized + ser :: Serialize > (& mut self , value : & T ,) -> Result < () , Error > { { let key = self . key . as_ref () . ok_or_else (Error :: no_key) ? ; let value_sink = value :: ValueSink :: new (self . urlencoder , & key) ; value . serialize (part :: PartSerializer :: new (value_sink)) ? ; } self . key = None ; Ok (()) } fn end (self) -> Result < Self :: Ok , Error > { Ok (self . urlencoder) } }
};
}
