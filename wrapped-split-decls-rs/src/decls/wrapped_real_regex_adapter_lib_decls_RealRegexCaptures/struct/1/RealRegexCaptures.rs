use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct RealRegexCaptures < 't > { captures : regex :: Captures < 't > , }
}