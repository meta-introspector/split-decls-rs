macro_rules! serde {
    () => {
        # [cfg (feature = "serde")] pub mod serde { # ! [doc = " Adapters for alternative `serde` formats."] # ! [doc = ""] # ! [doc = " This module contains adapters you can use with [`#[serde(with)]`](https://serde.rs/field-attrs.html#with)"] # ! [doc = " to change the way a [`Uuid`](../struct.Uuid.html) is serialized"] # ! [doc = " and deserialized."] pub use crate :: external :: serde_support :: { braced , compact , simple , urn } ; }
    };
}

serde!();