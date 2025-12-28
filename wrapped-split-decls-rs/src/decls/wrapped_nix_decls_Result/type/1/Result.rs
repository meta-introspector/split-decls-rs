use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " Nix Result Type"] pub type Result < T > = result :: Result < T , Errno > ;
}