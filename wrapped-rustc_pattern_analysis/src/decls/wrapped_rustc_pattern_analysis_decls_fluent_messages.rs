use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "rustc")]
rustc_fluent_macro::fluent_messages! {
    "../messages.ftl"
}
