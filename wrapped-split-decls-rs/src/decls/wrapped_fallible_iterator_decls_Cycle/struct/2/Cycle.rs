use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An iterator which cycles another endlessly."] # [derive (Clone , Debug)] pub struct Cycle < I > { it : I , cur : I , }
}