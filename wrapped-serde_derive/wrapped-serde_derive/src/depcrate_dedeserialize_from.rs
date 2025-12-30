// Generated macro for deserialize_from (function)
macro_rules! Depcrate_dedeserialize_from {
() => {
// Module: crate::de
// Provides: {"deserialize_from"}
// Dependencies: {}
# [doc = " Generates `Deserialize::deserialize` body for a type with `#[serde(from)]` attribute"] fn deserialize_from (type_from : & syn :: Type) -> Fragment { quote_block ! { _serde ::# private :: Result :: map (<# type_from as _serde :: Deserialize >:: deserialize (__deserializer) , _serde ::# private :: From :: from) } }
};
}
