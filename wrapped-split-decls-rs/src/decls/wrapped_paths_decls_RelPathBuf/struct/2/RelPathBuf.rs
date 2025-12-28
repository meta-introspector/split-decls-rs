use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Wrapper around a relative [`Utf8PathBuf`]."] # [derive (Debug , Clone , Ord , PartialOrd , Eq , PartialEq , Hash)] pub struct RelPathBuf (Utf8PathBuf) ;
}