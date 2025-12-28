use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " A list of crate items."] pub type CrateItems = Vec < CrateItem > ;
}