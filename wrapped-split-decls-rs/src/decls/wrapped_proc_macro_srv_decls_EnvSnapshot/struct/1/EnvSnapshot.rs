use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct EnvSnapshot { vars : HashMap < OsString , OsString > , }
}