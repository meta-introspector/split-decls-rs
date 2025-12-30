// Generated macro for impl_461 (impl)
macro_rules! Depcrate_ser_arrayimpl_461 {
() => {
// Module: crate::ser::array
// Provides: {"impl_461"}
// Dependencies: {}
impl serde_core :: ser :: SerializeTupleVariant for SerializeTupleVariant { type Ok = crate :: Value ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { serde_core :: ser :: SerializeSeq :: serialize_element (& mut self . inner , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { let inner = serde_core :: ser :: SerializeSeq :: end (self . inner) ? ; let mut items = crate :: table :: KeyValuePairs :: new () ; let value = crate :: Item :: Value (inner) ; items . insert (crate :: Key :: new (self . variant) , value) ; Ok (crate :: Value :: InlineTable (crate :: InlineTable :: with_pairs (items ,))) } }
};
}
