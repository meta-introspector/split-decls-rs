use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " Result type of the crate."] # [cfg (not (feature = "std"))] pub type Result < T > = core :: result :: Result < T , Error > ;
}