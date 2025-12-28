use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Figure size"] # [derive (Clone , Copy)] pub struct Size (pub usize , pub usize) ;
}