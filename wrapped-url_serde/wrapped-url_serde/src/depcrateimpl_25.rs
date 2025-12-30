// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for De < Host > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > { let string_representation : String = Deserialize :: deserialize (deserializer) ? ; Host :: parse (& string_representation) . map (De) . map_err (| err | { serde :: de :: Error :: custom (err . description ()) }) } }
};
}
