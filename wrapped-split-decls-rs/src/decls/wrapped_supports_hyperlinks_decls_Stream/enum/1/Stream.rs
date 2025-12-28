use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " possible stream sources"] # [derive (Clone , Copy , Debug)] pub enum Stream { Stdout , Stderr , }