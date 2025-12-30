// Generated macro for deserialize (function)
macro_rules! Depcratedeserialize {
() => {
// Module: crate
// Provides: {"deserialize"}
// Dependencies: {}
# [doc = " Deserialises a `T` value with a given deserializer."] # [doc = ""] # [doc = " This is useful to deserialize Url types used in structure fields or"] # [doc = " tuple members with `#[serde(deserialize_with = \"url_serde::deserialize\")]`."] pub fn deserialize < 'de , T , D > (deserializer : D) -> Result < T , D :: Error > where D : Deserializer < 'de > , De < T > : Deserialize < 'de > { De :: deserialize (deserializer) . map (De :: into_inner) }
};
}
