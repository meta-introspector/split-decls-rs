use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Wrapper around an absolute [`Utf8Path`]."] # [derive (Debug , Ord , PartialOrd , Eq , Hash)] # [repr (transparent)] pub struct AbsPath (Utf8Path) ;
}