use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Copy)] enum StrLitKind { Normal , Raw (usize) , }
}