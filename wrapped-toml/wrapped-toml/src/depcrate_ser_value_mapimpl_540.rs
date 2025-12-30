// Generated macro for impl_540 (impl)
macro_rules! Depcrate_ser_value_mapimpl_540 {
() => {
// Module: crate::ser::value::map
// Provides: {"impl_540"}
// Dependencies: {}
impl < 'd > serde_core :: ser :: SerializeStruct for SerializeTable < 'd > { type Ok = & 'd mut String ; type Error = Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { let mut encoded_value = String :: new () ; let mut is_none = false ; let value_serializer = MapValueSerializer :: new (& mut encoded_value , & mut is_none , self . style) ; let res = value . serialize (value_serializer) ; match res { Ok (_) => { use core :: fmt :: Write as _ ; if self . seen_value { self . dst . val_sep () ? ; } self . seen_value = true ; self . dst . space () ? ; self . dst . key (key) ? ; self . dst . space () ? ; self . dst . keyval_sep () ? ; self . dst . space () ? ; write ! (self . dst , "{encoded_value}") ? ; } Err (e) => { if ! (e == Error :: unsupported_none () && is_none) { return Err (e) ; } } } Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . end () } }
};
}
