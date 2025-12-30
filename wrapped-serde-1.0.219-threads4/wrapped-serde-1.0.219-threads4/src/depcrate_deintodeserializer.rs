// Generated macro for IntoDeserializer (trait)
macro_rules! Depcrate_deIntoDeserializer {
() => {
// Module: crate::de
// Provides: {"IntoDeserializer"}
// Dependencies: {}
# [doc = " Converts an existing value into a `Deserializer` from which other values can"] # [doc = " be deserialized."] # [doc = ""] # [doc = " # Lifetime"] # [doc = ""] # [doc = " The `'de` lifetime of this trait is the lifetime of data that may be"] # [doc = " borrowed from the resulting `Deserializer`. See the page [Understanding"] # [doc = " deserializer lifetimes] for a more detailed explanation of these lifetimes."] # [doc = ""] # [doc = " [Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```edition2021"] # [doc = " use serde::de::{value, Deserialize, IntoDeserializer};"] # [doc = " use serde_derive::Deserialize;"] # [doc = " use std::str::FromStr;"] # [doc = ""] # [doc = " #[derive(Deserialize)]"] # [doc = " enum Setting {"] # [doc = "     On,"] # [doc = "     Off,"] # [doc = " }"] # [doc = ""] # [doc = " impl FromStr for Setting {"] # [doc = "     type Err = value::Error;"] # [doc = ""] # [doc = "     fn from_str(s: &str) -> Result<Self, Self::Err> {"] # [doc = "         Self::deserialize(s.into_deserializer())"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub trait IntoDeserializer < 'de , E : Error = value :: Error > { # [doc = " The type of the deserializer being converted into."] type Deserializer : Deserializer < 'de , Error = E > ; # [doc = " Convert this value into a deserializer."] fn into_deserializer (self) -> Self :: Deserializer ; }
};
}
