use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
pub struct SerializeMetadata<'a>(&'a Metadata<'a>);
