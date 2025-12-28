use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct CompletionFieldsToResolve { pub resolve_label_details : bool , pub resolve_tags : bool , pub resolve_detail : bool , pub resolve_documentation : bool , pub resolve_filter_text : bool , pub resolve_text_edit : bool , pub resolve_command : bool , }