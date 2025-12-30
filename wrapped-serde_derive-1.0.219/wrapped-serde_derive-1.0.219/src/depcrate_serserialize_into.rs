// Generated macro for serialize_into (function)
macro_rules! Depcrate_serserialize_into {
() => {
// Module: crate::ser
// Provides: {"serialize_into"}
// Dependencies: {}
fn serialize_into (params : & Parameters , type_into : & syn :: Type) -> Fragment { let self_var = & params . self_var ; quote_block ! { _serde :: Serialize :: serialize (& _serde :: __private :: Into ::<# type_into >:: into (_serde :: __private :: Clone :: clone (# self_var)) , __serializer) } }
};
}
