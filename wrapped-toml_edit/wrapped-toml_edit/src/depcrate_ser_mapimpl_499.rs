// Generated macro for impl_499 (impl)
macro_rules! Depcrate_ser_mapimpl_499 {
() => {
// Module: crate::ser::map
// Provides: {"impl_499"}
// Dependencies: {}
impl serde_core :: ser :: SerializeStructVariant for SerializeStructVariant { type Ok = crate :: Value ; type Error = Error ; # [inline] fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , < Self as serde_core :: ser :: SerializeStructVariant > :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { serde_core :: ser :: SerializeStruct :: serialize_field (& mut self . inner , key , value) } # [inline] fn end (self) -> Result < < Self as serde_core :: ser :: SerializeStructVariant > :: Ok , < Self as serde_core :: ser :: SerializeStructVariant > :: Error > { let inner = serde_core :: ser :: SerializeStruct :: end (self . inner) ? . into () ; let mut items = crate :: table :: KeyValuePairs :: new () ; let value = crate :: Item :: Value (inner) ; items . insert (crate :: Key :: new (self . variant) , value) ; Ok (crate :: Value :: InlineTable (crate :: InlineTable :: with_pairs (items ,))) } }
};
}
