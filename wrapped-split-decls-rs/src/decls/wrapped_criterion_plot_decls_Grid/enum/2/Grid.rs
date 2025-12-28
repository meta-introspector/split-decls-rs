use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Grid line"] # [derive (Clone , Copy)] pub enum Grid { # [doc = " Major gridlines"] Major , # [doc = " Minor gridlines"] Minor , }
}