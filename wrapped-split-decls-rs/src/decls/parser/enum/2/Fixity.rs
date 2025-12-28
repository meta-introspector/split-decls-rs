use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (PartialEq , Debug)] pub enum Fixity { # [doc = " The operator is left-associative"] Left , # [doc = " The operator is right-associative"] Right , # [doc = " The operator is not associative"] None , }
}