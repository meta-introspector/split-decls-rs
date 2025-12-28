use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] pub struct SerializeField < 'a > (& 'a Field) ;
}