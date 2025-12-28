use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ExternalSource { pub fn get_source (& self) -> Option < & str > { match self { ExternalSource :: Foreign { kind : ExternalSourceKind :: Present (src) , .. } => Some (src) , _ => None , } } }