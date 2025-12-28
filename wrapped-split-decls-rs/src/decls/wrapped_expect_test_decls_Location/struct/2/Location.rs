use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] struct Location { line_indent : usize , # [doc = " The byte range of the argument to `expect!`, including the inner `[]` if it exists."] literal_range : Range < usize > , }
}