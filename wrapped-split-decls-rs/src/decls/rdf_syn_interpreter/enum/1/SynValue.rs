use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone)] pub enum SynValue { Path (PathBuf) , String (String) , Bool (bool) , Unit , }
}