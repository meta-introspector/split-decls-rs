// Generated macro for deserialize_try_from (function)
macro_rules! Depcrate_dedeserialize_try_from {
() => {
// Module: crate::de
// Provides: {"deserialize_try_from"}
// Dependencies: {}
fn deserialize_try_from (type_try_from : & syn :: Type) -> Fragment { quote_block ! { _serde :: __private :: Result :: and_then (<# type_try_from as _serde :: Deserialize >:: deserialize (__deserializer) , | v | _serde :: __private :: TryFrom :: try_from (v) . map_err (_serde :: de :: Error :: custom)) } }
};
}
