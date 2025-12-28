use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn all_names () -> Vec < & 'static str > { ExternAbi :: ALL_VARIANTS . iter () . map (| abi | abi . as_str ()) . collect () }