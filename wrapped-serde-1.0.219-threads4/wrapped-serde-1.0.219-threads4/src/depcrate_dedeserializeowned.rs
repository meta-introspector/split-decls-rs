// Generated macro for DeserializeOwned (trait)
macro_rules! Depcrate_deDeserializeOwned {
() => {
// Module: crate::de
// Provides: {"DeserializeOwned"}
// Dependencies: {}
# [doc = " A data structure that can be deserialized without borrowing any data from"] # [doc = " the deserializer."] # [doc = ""] # [doc = " This is primarily useful for trait bounds on functions. For example a"] # [doc = " `from_str` function may be able to deserialize a data structure that borrows"] # [doc = " from the input string, but a `from_reader` function may only deserialize"] # [doc = " owned data."] # [doc = ""] # [doc = " ```edition2021"] # [doc = " # use serde::de::{Deserialize, DeserializeOwned};"] # [doc = " # use std::io::{Read, Result};"] # [doc = " #"] # [doc = " # trait Ignore {"] # [doc = " fn from_str<'a, T>(s: &'a str) -> Result<T>"] # [doc = " where"] # [doc = "     T: Deserialize<'a>;"] # [doc = ""] # [doc = " fn from_reader<R, T>(rdr: R) -> Result<T>"] # [doc = " where"] # [doc = "     R: Read,"] # [doc = "     T: DeserializeOwned;"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " # Lifetime"] # [doc = ""] # [doc = " The relationship between `Deserialize` and `DeserializeOwned` in trait"] # [doc = " bounds is explained in more detail on the page [Understanding deserializer"] # [doc = " lifetimes]."] # [doc = ""] # [doc = " [Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html"] pub trait DeserializeOwned : for < 'de > Deserialize < 'de > { }
};
}
