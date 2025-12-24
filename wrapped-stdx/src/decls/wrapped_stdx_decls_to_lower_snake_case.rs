use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn to_lower_snake_case(s: &str) -> String {
    to_snake_case(s, char::to_lowercase)
}
