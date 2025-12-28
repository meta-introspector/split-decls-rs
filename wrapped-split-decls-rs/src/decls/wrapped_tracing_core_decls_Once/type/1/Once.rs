use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [cfg (not (feature = "std"))] # [doc (hidden)] pub type Once = self :: spin :: Once < () > ;
}