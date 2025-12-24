use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "serde")]
pub mod serde {
    //! Adapters for alternative `serde` formats.
    //!
    //! This module contains adapters you can use with [`#[serde(with)]`](https://serde.rs/field-attrs.html#with)
    //! to change the way a [`Uuid`](../struct.Uuid.html) is serialized
    //! and deserialized.
    pub use crate::external::serde_support::{braced, compact, simple, urn};
}
