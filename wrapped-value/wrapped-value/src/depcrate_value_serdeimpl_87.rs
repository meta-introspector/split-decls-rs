// Generated macro for impl_87 (impl)
macro_rules! Depcrate_value_serdeimpl_87 {
() => {
// Module: crate::value_serde
// Provides: {"impl_87"}
// Dependencies: {}
impl Serialize for Value { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { match self { Value :: Variable (name) => SerdeVariable (name . clone ()) . serialize (serializer) , Value :: Null => serializer . serialize_none () , Value :: Number (v) => v . serialize (serializer) , Value :: String (v) => serializer . serialize_str (v) , Value :: Boolean (v) => serializer . serialize_bool (* v) , Value :: Binary (v) => serializer . serialize_bytes (v) , Value :: Enum (v) => serializer . serialize_str (v) , Value :: List (v) => v . serialize (serializer) , Value :: Object (v) => v . serialize (serializer) , } } }
};
}
