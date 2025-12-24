use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Unsafe functions that mirror the API of the C implementation of Ryū.
pub mod raw {
    pub use crate::pretty::{format32, format64};
}
