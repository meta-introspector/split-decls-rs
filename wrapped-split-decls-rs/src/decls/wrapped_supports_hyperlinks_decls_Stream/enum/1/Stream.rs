use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " possible stream sources"] # [derive (Clone , Copy , Debug)] pub enum Stream { Stdout , Stderr , }
}