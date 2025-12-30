// Generated macro for impl_133 (impl)
macro_rules! Depcrate_cldr_serde_date_fieldsimpl_133 {
() => {
// Module: crate::cldr_serde::date_fields
// Provides: {"impl_133"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Field { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { deserializer . deserialize_map (FieldVisitor) } }
};
}
