use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Normalize the string to avoid any unicode control characters."] # [doc = ""] # [doc = " This is important for untrusted input, as it can contain"] # [doc = " invalid unicode sequences."] pub fn normalize_untrusted_str (s : & str) -> String { renderer :: normalize_whitespace (s) }
}