use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Serialize , Deserialize)] enum FileStatus { Ok , ParsingError , }
}