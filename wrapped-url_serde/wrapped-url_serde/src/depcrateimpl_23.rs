// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
# [doc = " Deserializes this URL from a `serde` stream."] impl < 'de > Deserialize < 'de > for De < Url > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > { let string_representation : String = Deserialize :: deserialize (deserializer) ? ; Url :: parse (& string_representation) . map (De) . map_err (| err | { serde :: de :: Error :: custom (err . description ()) }) } }
};
}
