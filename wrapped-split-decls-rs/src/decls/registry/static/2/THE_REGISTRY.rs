use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
# [doc = " ////////////////////////////////////////////////////////////////////////"] # [doc = " Initialization"] static mut THE_REGISTRY : Option < Arc < Registry > > = None ;
}