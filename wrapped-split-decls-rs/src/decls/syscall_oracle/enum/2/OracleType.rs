use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub enum OracleType { FileSystem , Network , Process , Memory , Time , Crypto , Custom (String) , }