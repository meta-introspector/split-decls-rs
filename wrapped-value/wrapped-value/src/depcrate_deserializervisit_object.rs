// Generated macro for visit_object (function)
macro_rules! Depcrate_deserializervisit_object {
() => {
// Module: crate::deserializer
// Provides: {"visit_object"}
// Dependencies: {}
fn visit_object < 'de , V > (object : IndexMap < Name , ConstValue > , visitor : V ,) -> Result < V :: Value , DeserializerError > where V : Visitor < 'de > , { let len = object . len () ; let mut deserializer = MapDeserializer :: new (object) ; let map = visitor . visit_map (& mut deserializer) ? ; let remaining = deserializer . iter . len () ; if remaining == 0 { Ok (map) } else { Err (DeserializerError :: invalid_length (len , & "fewer elements in map" ,)) } }
};
}
