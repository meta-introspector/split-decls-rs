use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug)] enum SymbolInfo { Function (String) , Type (String) , Constant (String) , }
}