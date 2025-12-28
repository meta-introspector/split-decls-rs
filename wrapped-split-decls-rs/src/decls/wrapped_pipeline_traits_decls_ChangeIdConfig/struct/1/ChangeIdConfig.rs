use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Deserialize , Clone)] pub struct ChangeIdConfig { pub id : Option < String > , }
}