use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Hash value newtype. Not larger than usize, since anything larger"] # [doc = " isn't used for selecting position anyway."] # [derive (Clone , Copy , Debug , PartialEq)] struct HashValue (usize) ;
}