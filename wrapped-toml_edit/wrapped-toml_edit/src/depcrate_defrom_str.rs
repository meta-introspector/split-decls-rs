// Generated macro for from_str (function)
macro_rules! Depcrate_defrom_str {
() => {
// Module: crate::de
// Provides: {"from_str"}
// Dependencies: {}
# [doc = " Deserializes a string into a type."] # [doc = ""] # [doc = " This function will attempt to interpret `s` as a TOML document and"] # [doc = " deserialize `T` from the document."] # [doc = ""] # [doc = " To deserializes TOML values, instead of documents, see [`ValueDeserializer`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde::Deserialize;"] # [doc = ""] # [doc = " #[derive(Deserialize)]"] # [doc = " struct Config {"] # [doc = "     title: String,"] # [doc = "     owner: Owner,"] # [doc = " }"] # [doc = ""] # [doc = " #[derive(Deserialize)]"] # [doc = " struct Owner {"] # [doc = "     name: String,"] # [doc = " }"] # [doc = ""] # [doc = " let config: Config = toml_edit::de::from_str(r#\""] # [doc = "     title = 'TOML Example'"] # [doc = ""] # [doc = "     [owner]"] # [doc = "     name = 'Lisa'"] # [doc = " \"#).unwrap();"] # [doc = ""] # [doc = " assert_eq!(config.title, \"TOML Example\");"] # [doc = " assert_eq!(config.owner.name, \"Lisa\");"] # [doc = " ```"] # [cfg (feature = "parse")] pub fn from_str < T > (s : & '_ str) -> Result < T , Error > where T : DeserializeOwned , { let de = Deserializer :: parse (s) ? ; T :: deserialize (de) }
};
}
