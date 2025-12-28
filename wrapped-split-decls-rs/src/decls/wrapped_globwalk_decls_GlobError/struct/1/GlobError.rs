use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Error from parsing globs."] # [derive (Debug)] pub struct GlobError (ignore :: Error) ;