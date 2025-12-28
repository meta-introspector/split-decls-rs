use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn determine_relation_type (symbol : & str) -> String { if symbol . contains ("module_not_found") { "missing_dependency" . to_string () } else if symbol . contains ("decls_") { "declaration_split" . to_string () } else if symbol . contains ("wrapped_") { "overlay_transformation" . to_string () } else if symbol . contains ("::") { "namespace_relation" . to_string () } else { "direct_usage" . to_string () } }