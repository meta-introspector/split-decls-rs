use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Eq , PartialEq)] enum SelfBounds < 'a > { None , All (& 'a [Trait]) , }
}