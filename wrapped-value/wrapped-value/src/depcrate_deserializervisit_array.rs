// Generated macro for visit_array (function)
macro_rules! Depcrate_deserializervisit_array {
() => {
// Module: crate::deserializer
// Provides: {"visit_array"}
// Dependencies: {}
fn visit_array < 'de , V > (array : Vec < ConstValue > , visitor : V) -> Result < V :: Value , DeserializerError > where V : Visitor < 'de > , { let len = array . len () ; let mut deserializer = SeqDeserializer :: new (array) ; let seq = visitor . visit_seq (& mut deserializer) ? ; let remaining = deserializer . iter . len () ; if remaining == 0 { Ok (seq) } else { Err (DeserializerError :: invalid_length (len , & "fewer elements in array" ,)) } }
};
}
