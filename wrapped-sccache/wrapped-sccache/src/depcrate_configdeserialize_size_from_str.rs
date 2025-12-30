// Generated macro for deserialize_size_from_str (function)
macro_rules! Depcrate_configdeserialize_size_from_str {
() => {
// Module: crate::config
// Provides: {"deserialize_size_from_str"}
// Dependencies: {}
fn deserialize_size_from_str < 'de , D > (deserializer : D) -> StdResult < u64 , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_any (StringOrU64Visitor) }
};
}
