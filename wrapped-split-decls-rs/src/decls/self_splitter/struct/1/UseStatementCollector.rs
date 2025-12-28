use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Default)] struct UseStatementCollector { uses : Vec < ItemUse > , }
}