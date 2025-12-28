use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn classify_integer_type (value : i32) -> String { if value > 0 && (value & (value - 1)) == 0 { "PowerOfTwo" . to_string () } else if value < 0 { "ErrorCode" . to_string () } else if value == 25519 || value == - 25519 || value == 256 || value == - 256 { "CryptoConstant" . to_string () } else if value >= - 10 && value <= 10 { "SmallInteger" . to_string () } else { "Integer" . to_string () } }