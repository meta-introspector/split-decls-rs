// Generated macro for to_value (function)
macro_rules! Depcrate_serializerto_value {
() => {
// Module: crate::serializer
// Provides: {"to_value"}
// Dependencies: {}
# [doc = " Convert a `T` into `ConstValue` which is an enum that can represent any"] # [doc = " valid GraphQL data."] # [inline] pub fn to_value < T : ser :: Serialize > (value : T) -> Result < ConstValue , SerializerError > { value . serialize (Serializer) }
};
}
