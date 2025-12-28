use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Serialize)] pub struct DuplicateReportXML < T > { pub items : T , }
}