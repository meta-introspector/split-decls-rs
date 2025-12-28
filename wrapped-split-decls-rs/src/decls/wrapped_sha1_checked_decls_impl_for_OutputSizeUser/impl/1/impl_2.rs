use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl OutputSizeUser for Sha1 { type OutputSize = U20 ; }