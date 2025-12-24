use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A type that provides internal information but that can still be used for debug purpose.
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Opaque(String);
