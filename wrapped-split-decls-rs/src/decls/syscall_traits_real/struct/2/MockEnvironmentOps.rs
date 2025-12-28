use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Default)] pub struct MockEnvironmentOps { pub vars : std :: collections :: HashMap < String , String > , }
}