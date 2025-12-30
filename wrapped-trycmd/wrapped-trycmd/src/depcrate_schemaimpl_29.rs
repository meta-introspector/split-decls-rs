// Generated macro for impl_29 (impl)
macro_rules! Depcrate_schemaimpl_29 {
() => {
// Module: crate::schema
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'de > serde :: de :: Deserialize < 'de > for JoinedArgs { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: de :: Deserializer < 'de > , { let s = String :: deserialize (deserializer) ? ; std :: str :: FromStr :: from_str (& s) . map_err (serde :: de :: Error :: custom) } }
};
}
