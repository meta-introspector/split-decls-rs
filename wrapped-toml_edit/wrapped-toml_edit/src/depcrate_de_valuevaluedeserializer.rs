// Generated macro for ValueDeserializer (struct)
macro_rules! Depcrate_de_valueValueDeserializer {
() => {
// Module: crate::de::value
// Provides: {"ValueDeserializer"}
// Dependencies: {}
# [doc = " Deserialization implementation for TOML [values][crate::Value]."] # [doc = ""] # [doc = " Can be created either directly from TOML strings, using [`std::str::FromStr`],"] # [doc = " or from parsed [values][crate::Value] using"] # [doc = " [`IntoDeserializer::into_deserializer`][serde_core::de::IntoDeserializer::into_deserializer]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(feature = \"parse\")] {"] # [doc = " # #[cfg(feature = \"display\")] {"] # [doc = " use serde::Deserialize;"] # [doc = ""] # [doc = " #[derive(Deserialize)]"] # [doc = " struct Config {"] # [doc = "     title: String,"] # [doc = "     owner: Owner,"] # [doc = " }"] # [doc = ""] # [doc = " #[derive(Deserialize)]"] # [doc = " struct Owner {"] # [doc = "     name: String,"] # [doc = " }"] # [doc = ""] # [doc = " let value = r#\"{ title = 'TOML Example', owner = { name = 'Lisa' } }\"#;"] # [doc = " let deserializer = value.parse::<toml_edit::de::ValueDeserializer>().unwrap();"] # [doc = " let config = Config::deserialize(deserializer).unwrap();"] # [doc = " assert_eq!(config.title, \"TOML Example\");"] # [doc = " assert_eq!(config.owner.name, \"Lisa\");"] # [doc = " # }"] # [doc = " # }"] # [doc = " ```"] pub struct ValueDeserializer { input : crate :: Item , validate_struct_keys : bool , }
};
}
