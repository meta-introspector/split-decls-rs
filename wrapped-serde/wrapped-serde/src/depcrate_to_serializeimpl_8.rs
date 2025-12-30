// Generated macro for impl_8 (impl)
macro_rules! Depcrate_to_serializeimpl_8 {
() => {
// Module: crate::to_serialize
// Provides: {"impl_8"}
// Dependencies: {}
impl < V : sval :: Value > serde_core :: Serialize for ToSerialize < V > { fn serialize < S : serde_core :: Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { Serializer :: new (serializer) . value_ref (& self . 0) . unwrap_or_else (| e | Err (S :: Error :: custom (e))) } }
};
}
