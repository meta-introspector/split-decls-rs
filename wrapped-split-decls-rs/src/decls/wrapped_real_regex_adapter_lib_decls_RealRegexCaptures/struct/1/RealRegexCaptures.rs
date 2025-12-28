use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct RealRegexCaptures < 't > { captures : regex :: Captures < 't > , }