use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn product_name(channel: String) -> String {
    format!(
        "Rust Compiler{}", if channel == "stable" { "".to_string() } else {
        format!(" ({})", channel) }
    )
}
