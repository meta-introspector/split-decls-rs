// Generated macro for serialize_unit_struct (function)
macro_rules! Depcrate_serserialize_unit_struct {
() => {
// Module: crate::ser
// Provides: {"serialize_unit_struct"}
// Dependencies: {}
fn serialize_unit_struct (cattrs : & attr :: Container) -> Fragment { let type_name = cattrs . name () . serialize_name () ; quote_expr ! { _serde :: Serializer :: serialize_unit_struct (__serializer , # type_name) } }
};
}
