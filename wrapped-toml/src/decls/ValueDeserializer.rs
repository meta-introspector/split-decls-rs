macro_rules! deps {
    () => {
        DeValue!();
    };
}

macro_rules! ValueDeserializer {
    () => {
        deps!();
        # [doc = " Deserialization implementation for TOML [values][crate::Value]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(feature = \"parse\")] {"] # [doc = " # #[cfg(feature = \"display\")] {"] # [doc = " use serde::Deserialize;"] # [doc = ""] # [doc = " #[derive(Deserialize)]"] # [doc = " struct Config {"] # [doc = "     title: String,"] # [doc = "     owner: Owner,"] # [doc = " }"] # [doc = ""] # [doc = " #[derive(Deserialize)]"] # [doc = " struct Owner {"] # [doc = "     name: String,"] # [doc = " }"] # [doc = ""] # [doc = " let value = r#\"{ title = 'TOML Example', owner = { name = 'Lisa' } }\"#;"] # [doc = " let deserializer = toml::de::ValueDeserializer::parse(value).unwrap();"] # [doc = " let config = Config::deserialize(deserializer).unwrap();"] # [doc = " assert_eq!(config.title, \"TOML Example\");"] # [doc = " assert_eq!(config.owner.name, \"Lisa\");"] # [doc = " # }"] # [doc = " # }"] # [doc = " ```"] pub struct ValueDeserializer < 'i > { span : core :: ops :: Range < usize > , input : DeValue < 'i > , validate_struct_keys : bool , }
    };
}

ValueDeserializer!()