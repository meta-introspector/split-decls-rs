use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " `Id` is a Graphviz `ID`."] pub struct Id < 'a > { name : Cow < 'a , str > , }
}