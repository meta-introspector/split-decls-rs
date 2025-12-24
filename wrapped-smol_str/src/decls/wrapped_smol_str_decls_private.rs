use serde::{Deserialize, Serialize};
use std::collections::HashMap;
mod private {
    /// No downstream impls allowed.
    pub trait Sealed {}
    impl Sealed for str {}
}
