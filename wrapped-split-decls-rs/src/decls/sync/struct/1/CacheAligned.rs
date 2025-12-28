use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Default)] # [repr (align (64))] pub struct CacheAligned < T > (pub T) ;
}