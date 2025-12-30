// Generated macro for to_value (function)
macro_rules! Depcrate_serto_value {
() => {
// Module: crate::ser
// Provides: {"to_value"}
// Dependencies: {}
pub fn to_value < T : ser :: Serialize > (value : T) -> Result < Value , SerializerError > { value . serialize (Serializer) }
};
}
