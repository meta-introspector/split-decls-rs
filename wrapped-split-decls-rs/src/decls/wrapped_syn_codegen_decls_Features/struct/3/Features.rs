use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Features behind which a syntax tree type is cfg gated."] # [derive (Clone , Debug , Default , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Features { # [doc = " Type is accessible if at least one of these features is enabled against"] # [doc = " the Syn dependency."] pub any : BTreeSet < String > , }
}