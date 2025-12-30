// Generated macro for impl_259 (impl)
macro_rules! Depcrate_schemaimpl_259 {
() => {
// Module: crate::schema
// Provides: {"impl_259"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Schema { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let value = Value :: deserialize (deserializer) ? ; Schema :: validate (& value) ? ; Ok (Schema (value)) } }
};
}
