use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn infer_field_type (symbol : & str , integer : i32) -> String { if integer == 0 || integer == 1 || integer == - 1 { "flag_or_boolean" } else if integer > 0 && (integer & (integer - 1)) == 0 { "power_of_two" } else if integer < 0 { "error_code_or_offset" } else if integer > 1000 { "large_constant" } else if integer > 100 { "medium_constant" } else { "small_integer" } . to_string () }