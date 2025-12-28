use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ExpandedForeignItem (pub Box < ast :: ForeignItem >) ;