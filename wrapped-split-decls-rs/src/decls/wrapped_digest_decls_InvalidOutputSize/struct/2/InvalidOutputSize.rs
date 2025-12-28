use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " The error type used in variable hash traits."] # [derive (Clone , Copy , Debug , Default)] pub struct InvalidOutputSize ;
}