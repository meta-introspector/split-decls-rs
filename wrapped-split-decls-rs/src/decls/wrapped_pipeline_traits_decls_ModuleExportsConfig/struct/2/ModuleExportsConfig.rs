use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Deserialize , Clone)] pub struct ModuleExportsConfig { pub modules : Option < Vec < String > > , }
}