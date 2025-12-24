use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "std")]
impl From<Uuid> for std::vec::Vec<u8> {
    fn from(value: Uuid) -> Self {
        value.0.to_vec()
    }
}
