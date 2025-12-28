use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn extract_usage_context (symbol : & str) -> String { if symbol . contains ("rustc_") { "compiler_internals" } else if symbol . contains ("std::") || symbol . contains ("core::") { "standard_library" } else if symbol . contains ("crypto") || symbol . contains ("hash") { "cryptography" } else if symbol . contains ("net") || symbol . contains ("io") { "networking_io" } else if symbol . contains ("test") { "testing_framework" } else if symbol . contains ("wrapped_") { "overlay_system" } else { "application_code" } . to_string () }
}