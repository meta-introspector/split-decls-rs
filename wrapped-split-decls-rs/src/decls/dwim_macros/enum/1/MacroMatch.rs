use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug)] enum MacroMatch { Single (MacroDefinition) , Ambiguous (Vec < MacroDefinition >) , None , }
}