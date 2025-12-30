// Generated macro for deserialize_try_from (function)
macro_rules! Depcrate_dedeserialize_try_from {
() => {
// Module: crate::de
// Provides: {"deserialize_try_from"}
// Dependencies: {}
# [doc = " Generates `Deserialize::deserialize` body for a type with `#[serde(try_from)]` attribute"] fn deserialize_try_from (type_try_from : & syn :: Type) -> Fragment { quote_block ! { _serde ::# private :: Result :: and_then (<# type_try_from as _serde :: Deserialize >:: deserialize (__deserializer) , | v | _serde ::# private :: TryFrom :: try_from (v) . map_err (_serde :: de :: Error :: custom)) } }
};
}
