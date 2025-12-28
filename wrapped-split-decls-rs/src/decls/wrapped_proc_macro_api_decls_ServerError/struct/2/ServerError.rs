use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Represents errors encountered when communicating with the proc-macro server."] # [derive (Clone , Debug)] pub struct ServerError { pub message : String , pub io : Option < Arc < io :: Error > > , }
}