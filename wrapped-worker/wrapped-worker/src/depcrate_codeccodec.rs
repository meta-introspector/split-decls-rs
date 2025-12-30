// Generated macro for Codec (trait)
macro_rules! Depcrate_codecCodec {
() => {
// Module: crate::codec
// Provides: {"Codec"}
// Dependencies: {}
# [doc = " Message Encoding and Decoding Format"] pub trait Codec { # [doc = " Encode an input to JsValue"] fn encode < I > (input : I) -> JsValue where I : Serialize ; # [doc = " Decode a message to a type"] fn decode < O > (input : JsValue) -> O where O : for < 'de > Deserialize < 'de > ; }
};
}
