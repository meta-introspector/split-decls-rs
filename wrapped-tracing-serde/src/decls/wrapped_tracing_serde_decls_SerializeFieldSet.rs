use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
pub struct SerializeFieldSet<'a>(&'a FieldSet);
