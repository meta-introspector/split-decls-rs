// Generated macro for hex_string (module)
macro_rules! Depcrate_wycheproofhex_string {
() => {
// Module: crate::wycheproof
// Provides: {"hex_string"}
// Dependencies: {}
pub mod hex_string { # ! [doc = " Manual JSON deserialization implementation for hex strings."] use serde :: Deserialize ; pub fn deserialize < 'de , D : serde :: Deserializer < 'de > > (deserializer : D ,) -> Result < Vec < u8 > , D :: Error > { let s = String :: deserialize (deserializer) ? ; :: hex :: decode (& s) . map_err (| _e | { serde :: de :: Error :: invalid_value (serde :: de :: Unexpected :: Str (& s) , & "hex data expected") }) } }
};
}
