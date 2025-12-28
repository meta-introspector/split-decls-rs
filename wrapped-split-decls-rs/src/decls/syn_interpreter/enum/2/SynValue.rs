use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] pub enum SynValue { Path (PathBuf) , String (String) , Bool (bool) , Result (Box < SynValue >) , Unit , }