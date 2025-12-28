use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline (always)] fn checked_add (num : usize , opt : Option < usize >) -> Option < usize > { if let Some (n) = opt { n . checked_add (num) } else { None } }
}