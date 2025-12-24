use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
pub struct SerializeLevel<'a>(&'a Level);
